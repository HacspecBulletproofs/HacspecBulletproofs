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
use hacspec_pedersen::*;

type RangeProofRes = Result<(RangeProof, Seq<RistrettoPointEncoded>), u8>;

type RangeProof = (
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
    mut transcript: Transcript,
    values: Seq<u64>,
    blindings: Seq<Scalar>,
    n: usize,
    a_blinding: Seq<Scalar>,
    s_blinding: Seq<Scalar>,
    s_L: Seq<Seq<Scalar>>,
    s_R: Seq<Seq<Scalar>>,
    t1_blinding: Seq<Scalar>,
    t2_blinding: Seq<Scalar>)
    -> RangeProofRes {
        let mut res = RangeProofRes::Err(0u8);
        if values.len() != blindings.len() {
            res = RangeProofRes::Err(WRONG_NUMBER_OF_BLINDINGS);
        }
        else{ if !(n == 8 || n == 16 || n == 32 || n == 64) {
            res = RangeProofRes::Err(INVALID_BIT_SIZE);
        }
        else{
            let (base_point, blinding_point) = pc_gens;
            let (party_capacity, gens_capacity, g_vec, h_vec) = bp_gens;
            let number_of_parties = values.len();
            /* Create dealer */

            let dealer_awaiting_bit_commitment = create_dealer((party_capacity, gens_capacity, g_vec.clone(), h_vec.clone()),pc_gens,transcript,n, number_of_parties)?;
            
            /* Create parties */

            let mut parties_waiting_for_position = Seq::<PartyAwaitingPosition>::new(number_of_parties);
            for i in 0..number_of_parties {
                parties_waiting_for_position[i] = create_party((party_capacity, gens_capacity, g_vec.clone(), h_vec.clone()),pc_gens, values[i],blindings[i],n)?;
            }
            
            /* Create a bitcommitment */

            let mut parties_waiting_for_bits = Seq::<PartyAwaitingBitChallenge>::new(number_of_parties);
            let mut bit_commitments = Seq::<(RistrettoPointEncoded,RistrettoPoint,RistrettoPoint)>::new(number_of_parties);
            for i in 0..number_of_parties {
                let (party, bit_commitment) = create_bit_commitment(parties_waiting_for_position[i].clone(),i, a_blinding[i], s_blinding[i],s_L[i].clone(),s_R[i].clone())?;
                parties_waiting_for_bits[i] = party;
                bit_commitments[i] = bit_commitment;
            }

            /* Create Bit Challenge */

            let mut value_commitments = Seq::<RistrettoPointEncoded>::new(number_of_parties);

            for i in 0..number_of_parties {
                let (v,_,_) = bit_commitments[i];
                value_commitments[i] = v;
            }

            let (dealer_awaiting_poly_commitments, bit_challenge) = receive_bit_commitments(dealer_awaiting_bit_commitment, bit_commitments)?;

            let mut parties_waiting_for_poly_challenge = Seq::<PartyAwaitingPolyChallenge>::new(number_of_parties);
            let mut poly_commitments = Seq::<(RistrettoPoint,RistrettoPoint)>::new(number_of_parties);

            for i in 0..number_of_parties{
                let (party_awaiting_poly_challenge, poly_commitment) = create_poly_commitment(parties_waiting_for_bits[i].clone(),bit_challenge,t1_blinding[i],t2_blinding[i])?;
                parties_waiting_for_poly_challenge[i] = party_awaiting_poly_challenge;
                poly_commitments[i] = poly_commitment;
            }

            let (dealer_awaiting_proof_shares, poly_challenge) = receive_poly_commitments(dealer_awaiting_poly_commitments, poly_commitments)?;

            let mut proofshares = Seq::<ProofShare>::new(number_of_parties);

            for i in 0..number_of_parties{
                proofshares[i] = create_proofshare(parties_waiting_for_poly_challenge[i].clone(), poly_challenge)?;
            }

            let proof = receive_shares(dealer_awaiting_proof_shares, proofshares)?;

            res = RangeProofRes::Ok((proof,value_commitments));
            
        }}
        res
    }
    

        

        /*

        let (dealer_awaiting_polycommitment, bit_challenge) = create_bit_challenge(dealer_awaiting_bit_commitment, bit_commitment);

        let (party_awaiting_poly_challenge, poly_commitment) = create_polycommitment(party_awaiting_bit_challenge, bit_challenge);

        let (dealer_awaiting_proofshare, poly_challenge) = create_polychallenge(dealer_awaiting_poly_commitment, poly_commitment);

        let proofshare = create_proofshare(party_awaiting_poly_challenge,poly_challenge);

        let range_proof = create_range_proof(dealer_awaiting_proofshare, proofshare);*/
