#![allow(non_snake_case)]
mod transcript;
mod dealer;
mod party;
mod errors;
mod types;
//use transcript::*;
use dealer::*;
use party::*;
use errors::*;
use types::*;

use hacspec_lib::*;
use hacspec_merlin::*;
use hacspec_ristretto::*;
use hacspec_ipp::*;
//use hacspec_pedersen::*;

type RangeProofRes = Result<(RangeProof, Seq<RistrettoPointEncoded>), u8>;

pub type RangeProof = (
    /* Commitment to the bits of the value*/
    /*A:*/ RistrettoPointEncoded,
    /*Commitment to the blinding factors*/
    /*S:*/ RistrettoPointEncoded,
    /* Commitment to the \\(t_1\\) coefficient of \\( t(x) \\)*/
    /*T_1:*/ RistrettoPointEncoded,
    /* Commitment to the \\(t_2\\) coefficient of \\( t(x) \\)*/
    /*T_2:*/ RistrettoPointEncoded,
    /* Evaluation of the polynomial \\(t(x)\\) at the challenge point \\(x\\)*/
    /*t_x:*/ Scalar,
    /*Blinding factor for the synthetic commitment to \\(t(x)\\)*/
    /*t_x_blinding:*/ Scalar,
    /* Blinding factor for the synthetic commitment to the inner-product arguments*/
    /*e_blinding:*/ Scalar,
    /* Proof data for the inner-product argument.*/
    /*ipp_proof:*/ InnerProductProof
);

/* --- MAIN METHODS --- */

pub fn prove(
    bp_gens: BulletproofGens,
    pc_gens: PedersenGens,
    transcript: Transcript,
    values: Seq<u64>,
    v_blindings: Seq<Scalar>,
    n: usize,
    a_blinding: Seq<Scalar>,
    s_blinding: Seq<Scalar>,
    s_L: Seq<Seq<Scalar>>,
    s_R: Seq<Seq<Scalar>>,
    t1_blinding: Seq<Scalar>,
    t2_blinding: Seq<Scalar>)
    -> RangeProofRes {

        #[allow(unused)]
        let mut res =  RangeProofRes::Err(0u8);

        if values.len() != v_blindings.len() {
            res = RangeProofRes::Err(WRONG_NUMBER_OF_BLINDINGS);
        }
        else{ if !(n == 8 || n == 16 || n == 32 || n == 64) {
            res = RangeProofRes::Err(INVALID_BIT_SIZE);
        }
        else{
            
            let (party_capacity, gens_capacity, g_vec, h_vec) = bp_gens;
            let number_of_parties = values.len();
            /* Create dealer */

            let dealer_awaiting_bit_commitment = create_dealer((party_capacity, gens_capacity, g_vec.clone(), h_vec.clone()),pc_gens,transcript,n, number_of_parties)?;
            
            /* Create parties */
            /* Party awaiting position = (bp_gens, pc_gens, n, v, v_blinding, V) 
            this needs to be unpacked to avoid borrows and cloning errors from hacspec 
            similarly bp_gens also needs to be unpacked for the same reasons*/

            let mut bp_genss_party_capacity = Seq::<usize>::new(number_of_parties);
            let mut bp_genss_gens_capacity = Seq::<usize>::new(number_of_parties);
            let mut bp_genss_g_vec = Seq::<Seq<Seq<RistrettoPoint>>>::new(number_of_parties);
            let mut bp_genss_h_vec = Seq::<Seq<Seq<RistrettoPoint>>>::new(number_of_parties);
            let mut pc_genss = Seq::<PedersenGens>::new(number_of_parties);
            let mut Vs = Seq::<RistrettoPointEncoded>::new(number_of_parties);

            for i in 0..number_of_parties {
                let ((new_party_capacity, new_gens_capacity, new_g_vec, new_h_vec),new_pc_gens,new_V) = 
                    create_party((party_capacity, gens_capacity, g_vec.clone(), h_vec.clone()),pc_gens, values[i], v_blindings[i],n)?;
                
                bp_genss_party_capacity[i] = new_party_capacity;
                bp_genss_gens_capacity[i] = new_gens_capacity;
                bp_genss_g_vec[i] = new_g_vec;
                bp_genss_h_vec[i] = new_h_vec;
                pc_genss[i] = new_pc_gens;
                Vs[i] = new_V;

            }
            
            /* Create a bitcommitment */
            /* Party_awaiting_bit_challenge = (n, v, v_blinding, j, pc_gens, a_blinding, s_blinding, s_L, s_R)
            this once more needs to be unpacked. 
            Note that this differs from the rust implementation due to not using certain inputs, making them extranuous */

            let mut bit_commitments = Seq::<(RistrettoPointEncoded,RistrettoPoint,RistrettoPoint)>::new(number_of_parties);
            for i in 0..number_of_parties {

                let bit_commitment = 
                        create_bit_commitment(((bp_genss_party_capacity[i], bp_genss_gens_capacity[i],bp_genss_g_vec[i].clone(),bp_genss_h_vec[i].clone()),
                                                pc_genss[i],
                                                Vs[i]),
                                                values[i],
                                                n,
                                                i, a_blinding[i], s_blinding[i],s_L[i].clone(),s_R[i].clone())?;

                bit_commitments[i] = bit_commitment;
            }

            /* Create Bit Challenge */

            let mut value_commitments = Seq::<RistrettoPointEncoded>::new(number_of_parties);

            for i in 0..number_of_parties {
                let (v,_,_) = bit_commitments[i];
                value_commitments[i] = v;
            }

            let (dealer_awaiting_poly_commitments, bit_challenge) = receive_bit_commitments(dealer_awaiting_bit_commitment, bit_commitments)?;

            /* Create poly commitments */
            /* party awaiting poly challenge = (offset_zz, l_poly, r_poly, t_poly, v_blinding, a_blinding, s_blinding, t_1_blinding, t_2_blinding) 
            As before this is unpacked to avoid trouble with hacspec compliance*/

            let mut offsets = Seq::<Scalar>::new(number_of_parties);
            let mut l_poly0s = Seq::<Seq<Scalar>>::new(number_of_parties);
            let mut l_poly1s = Seq::<Seq<Scalar>>::new(number_of_parties);
            let mut r_poly0s = Seq::<Seq<Scalar>>::new(number_of_parties);
            let mut r_poly1s = Seq::<Seq<Scalar>>::new(number_of_parties);
            let mut t_polys = Seq::<(Scalar,Scalar,Scalar)>::new(number_of_parties);

            let mut poly_commitments = Seq::<(RistrettoPoint,RistrettoPoint)>::new(number_of_parties);

            for i in 0..number_of_parties{
                let ((new_offset, (new_l_poly0, new_l_poly1), (new_r_poly0,new_r_poly1), new_t_poly), 
                      poly_commitment) = 
                        create_poly_commitment((i,
                                               values[i],
                                               n, 
                                               bit_challenge,
                                               pc_genss[i],
                                               t1_blinding[i],
                                               t2_blinding[i],
                                               s_L[i].clone(),
                                               s_R[i].clone()))?;

                offsets[i] = new_offset;
                l_poly0s[i] = new_l_poly0;
                l_poly1s[i] = new_l_poly1;
                r_poly0s[i] = new_r_poly0;
                r_poly1s[i] = new_r_poly1;
                t_polys[i] = new_t_poly;

                poly_commitments[i] = poly_commitment;
            }

            let (dealer_awaiting_proof_shares, poly_challenge) = receive_poly_commitments(dealer_awaiting_poly_commitments, poly_commitments)?;

            let mut t_xs = Seq::<Scalar>::new(number_of_parties);
            let mut t_x_blindings = Seq::<Scalar>::new(number_of_parties);
            let mut e_blindings = Seq::<Scalar>::new(number_of_parties);
            let mut l_vecs = Seq::<Seq<Scalar>>::new(number_of_parties);
            let mut r_vecs = Seq::<Seq<Scalar>>::new(number_of_parties);


            for i in 0..number_of_parties{
                let (new_t_x,new_t_x_blinding,new_e_blinding,new_l_vec, new_r_vec) = 
                    create_proofshare((offsets[i], 
                                        (l_poly0s[i].clone(),l_poly1s[i].clone()), 
                                        (r_poly0s[i].clone(), r_poly1s[i].clone()), 
                                        t_polys[i]),
                                        v_blindings[i],
                                        a_blinding[i], 
                                        s_blinding[i],
                                        t1_blinding[i],
                                        t2_blinding[i], 
                                        poly_challenge)?;
                t_xs[i] = new_t_x;
                t_x_blindings[i] = new_t_x_blinding;
                e_blindings[i] = new_e_blinding;
                l_vecs[i] = new_l_vec;
                r_vecs[i] = new_r_vec;
            }

            let proof = receive_shares(dealer_awaiting_proof_shares, t_xs,t_x_blindings,e_blindings,l_vecs,r_vecs)?;

            res = RangeProofRes::Ok((proof,value_commitments));
            
        }}
        res
    }
