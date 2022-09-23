#![allow(non_snake_case)]
mod transcript;
mod dealer;
mod party;
mod errors;
mod types;
use transcript::*;
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
type VerifyRes = Result<(),u8>;

pub type RangeProof = (
    /* Commitment to the bits of the value*/
    RistrettoPointEncoded, // A
    /*Commitment to the blinding factors*/
    RistrettoPointEncoded, // S
    /* Commitment to the \\(t_1\\) coefficient of \\( t(x) \\)*/
    RistrettoPointEncoded, // T_1
    /* Commitment to the \\(t_2\\) coefficient of \\( t(x) \\)*/
    RistrettoPointEncoded, // T_2
    /* Evaluation of the polynomial \\(t(x)\\) at the challenge point \\(x\\)*/
    Scalar, // t_x
    /*Blinding factor for the synthetic commitment to \\(t(x)\\)*/
    Scalar, // t_x_blinding
    /* Blinding factor for the synthetic commitment to the inner-product arguments*/
    Scalar, // e_blinding
    /* Proof data for the inner-product argument.*/
    InnerProductProof
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
        let mut res = RangeProofRes::Err(0u8);

        if values.len() != v_blindings.len() {
            res = RangeProofRes::Err(WRONG_NUMBER_OF_BLINDINGS);
        }
        else { if !(n == 8 || n == 16 || n == 32 || n == 64) {
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
            let mut parties = Seq::<PartyAwaitingPosition>::new(number_of_parties);
            
            for i in 0..number_of_parties {
                let party = 
                    create_party((party_capacity, gens_capacity, g_vec.clone(), h_vec.clone()),pc_gens, values[i], v_blindings[i],n)?;
                parties[i] = party;
            }
            
            /* Create a bitcommitment */
            /* Party_awaiting_bit_challenge = (n, v, v_blinding, j, pc_gens, a_blinding, s_blinding, s_L, s_R)
            this once more needs to be unpacked. 
            Note that this differs from the rust implementation due to not using certain inputs, making them extranuous */

            let mut bit_commitments = Seq::<(RistrettoPointEncoded,RistrettoPoint,RistrettoPoint)>::new(number_of_parties);
            for i in 0..number_of_parties {

                let bit_commitment = 
                        create_bit_commitment(parties[i].clone(),
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

            let mut poly_challenge_parties = Seq::<PartyAwaitingPolyChallenge>::new(number_of_parties);
            let mut poly_commitments = Seq::<(RistrettoPoint,RistrettoPoint)>::new(number_of_parties);

            for i in 0..number_of_parties{
                let (new_party, poly_commitment) = 
                        create_poly_commitment(BitChallengeInput(i,
                                               values[i],
                                               n, 
                                               bit_challenge,
                                               pc_gens,
                                               t1_blinding[i],
                                               t2_blinding[i],
                                               s_L[i].clone(),
                                               s_R[i].clone()))?;

                poly_challenge_parties[i] = new_party.clone();

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
                    create_proofshare(poly_challenge_parties[i].clone(),
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

            res = RangeProofRes::Ok((proof, value_commitments));
            
        }}
        res
    }

pub fn verify(
    proof: RangeProof, 
    bp_gens: BulletproofGens, 
    pc_gens: PedersenGens, 
    mut transcript: Transcript, 
    value_commitments: Seq<RistrettoPointEncoded>,
    n: usize,
    c: Scalar)
    -> VerifyRes {
    
    #[allow(unused)]
    let mut res = VerifyRes::Err(0u8);

    let m = value_commitments.len();

    let (party_capacity, gens_capacity, g_vec, h_vec) = bp_gens;

    if !(n == 8 || n == 16 || n == 32 || n == 64) {
        res = VerifyRes::Err(INVALID_BIT_SIZE);
    }
    else{ if gens_capacity < n || party_capacity < m {
        res = VerifyRes::Err(INVALID_GENERATORS_LENGTH)
    }
    else {
        transcript = rangeproof_domain_sep(transcript,U64::classify(n as u64),U64::classify(m as u64));

        for i in 0..value_commitments.len() { 
            transcript = append_point(transcript, byte_seq!(86u8), value_commitments[i]);
        }

        let (A, S, T_1, T_2, t_x, t_x_blinding, e_blinding, (a, b, L_vec, R_vec)) = proof;

        transcript = validate_and_append_point(transcript, byte_seq!(65u8), A)?;
        transcript = validate_and_append_point(transcript, byte_seq!(83u8), S)?;

        let (transcript, y) = challenge_scalar(transcript, byte_seq!(121u8));
        let (mut transcript, z) = challenge_scalar(transcript, byte_seq!(122u8));

        let zz = z * z;
        let minus_z = Scalar::from_literal(0u128) - z;

        transcript = validate_and_append_point(transcript, byte_seq!(84u8, 95u8, 49u8), T_1)?;
        transcript = validate_and_append_point(transcript, byte_seq!(84u8, 95u8, 50u8), T_2)?;

        let (mut transcript, x) = challenge_scalar(transcript, byte_seq!(120u8));

        transcript = append_scalar(transcript, byte_seq!(116u8, 95u8, 120u8), t_x);
        transcript = append_scalar(transcript, byte_seq!(116u8, 95u8, 120u8, 95u8, 98u8, 108u8, 105u8, 110u8, 100u8, 105u8, 110u8, 103u8), t_x_blinding);
        transcript = append_scalar(transcript, byte_seq!(101u8, 95u8, 98u8, 108u8, 105u8, 110u8, 100u8, 105u8, 110u8, 103u8), e_blinding);

        let (transcript, w) = challenge_scalar(transcript, byte_seq!(119u8));

        let (x_sq, x_inv_sq, s) = verification_scalars((a, b, L_vec.clone(), R_vec.clone()), n * m, transcript)?;

        let mut s_inv = Seq::<Scalar>::new(s.len());

        for i in 0..s.len() {
            s_inv[i] = s[s.len()-1-i];
        }

        let mut powers_of_2 = Seq::<Scalar>::new(n);
        let two = Scalar::from_literal(2u128);
        powers_of_2[0] = Scalar::from_literal(1u128);
        for i in 1..n {
            powers_of_2[i] = two * powers_of_2[i-1usize];
        }

        let mut z_and_2 = Seq::<Scalar>::new(n*m);
        let mut z_exp = Scalar::from_literal(1u128);
        for i in 0..m {
            for j in 0..n {
                z_and_2[n*i + j] = z_exp * powers_of_2[j];
            }
            z_exp = z_exp * z;
        }

        let mut g = Seq::<Scalar>::new(s.len());
        let mut h = Seq::<Scalar>::new(s.len());
        let mut y_inv_exp = Scalar::from_literal(1u128);

        for i in 0..s.len(){

            g[i] = minus_z - a * s[i];

            h[i] = z + y_inv_exp * (zz * z_and_2[i] - b * s_inv[i]);
            y_inv_exp = y.inv() * y_inv_exp;
        }

        let mut value_commitment_scalars = Seq::<Scalar>::new(m);
        let mut z_exp = Scalar::from_literal(1u128);
        for i in 0..m {
            value_commitment_scalars[i] = c * zz * z_exp;
            z_exp = z_exp * z;
        }

        let sum_y = sum_of_powers(y, n*m);
        let sum_2 = sum_of_powers(Scalar::from_literal(2u128),n);
        let sum_z = sum_of_powers(z,m);
        let delta = (z - z * z) * sum_y - z * z * z * sum_2 * sum_z;
        
        let basepoint_scalar = w * (t_x - a * b) + c * (delta - t_x);

        let (base_point, blinding_point) = pc_gens;

        let A_decoded = decode(A)?;
        let S_decoded = decode(S)?;
        let T_1_decoded = decode(T_1)?;
        let T_2_decoded = decode(T_2)?;

        let mut mega_check = add(A_decoded, 
                         add(mul(x, S_decoded),
                         add(mul(c*x, T_1_decoded),
                         add(mul(c*x*x, T_2_decoded),
                         add(mul((Scalar::from_literal(0u128)-e_blinding) - c* t_x_blinding, blinding_point),
                         mul(basepoint_scalar, base_point)
                         ))))); //MEGA-possible there is an error here

        for i in 0..L_vec.len() {
            let L_vec_i = decode(L_vec[i])?;
            let R_vec_i = decode(R_vec[i])?;
            mega_check = add(mega_check, mul(x_sq[i], L_vec_i));
            mega_check = add(mega_check, mul(x_inv_sq[i], R_vec_i));
        }

        let mut G = Seq::<RistrettoPoint>::new(n*m);
        let mut H = Seq::<RistrettoPoint>::new(n*m);

        for i in 0..m {
            let g_vec_i = g_vec[i].clone();
            let h_vec_i = h_vec[i].clone();
            for j in 0..n {
                G[n*i + j] = g_vec_i[j];
                H[n*i + j] = h_vec_i[j];

            }
        }

        for i in 0..n*m {
            mega_check = add(mega_check, mul(g[i], G[i]));
            mega_check = add(mega_check, mul(h[i], H[i]));
        }

        for i in 0..value_commitments.len() {
            let value = decode(value_commitments[i])?;
            mega_check = add(mega_check, mul(value_commitment_scalars[i], value));
        }

        if equals(mega_check,IDENTITY_POINT()) {
            res = VerifyRes::Ok(());
        }
        else { 
            res = VerifyRes::Err(VERIFICATION_ERROR);
        }}}
    res
    }

    fn sum_of_powers(x: Scalar, n: usize) -> Scalar {
        let mut res = Scalar::from_literal(0u128);
        let mut x_exp = Scalar::from_literal(1u128);
        for _ in 0..n {
            res = res + x_exp;
            x_exp = x_exp * x;
        }
        res
    }
