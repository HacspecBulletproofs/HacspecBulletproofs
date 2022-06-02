mod transcript;
use hacspec_lib::*;
use hacspec_merlin::*;
use hacspec_ristretto::*;
use hacspec_ipp::*;
use transcript::*;
use hacspec_pedersen::*;

const INVALID_BIT_SIZE: u8 = 50u8;

type RangeProofRes = Result<(RangeProof, Vec<RistrettoPointEncoded>), u8>;

type PedersenGens = ( 
    /*Base point:*/ RistrettoPoint, 
    /*Blinding point:*/ RistrettoPoint
);

type BulletproofGens = (
    /*Gens capacity*/ usize,
    /* Party capacity: */ usize, 
    /*Precomputed G-generators for each party:*/ 
    /* g_vec: */Vec<Vec<RistrettoPoint>>, 
    /*Precomputed H-generators for each party:*/ 
    /* h_vec: */Vec<Vec<RistrettoPoint>>
);

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
    value: u64,
    blinding: Scalar,
    n: usize)
    /*-> RangeProofRes*/ {
        let mut res = RangeProofRes::Err(0);

        let (base_point, blinding_point) = pc_gens;
        let (gens_capacity, party_capacity, g_vec, h_vec) = bp_gens;

        if !(n == 8 || n == 16 || n == 32 || n == 64) {
            res = RangeProofRes::Err(INVALID_BIT_SIZE);
        }
        else{
            /* Create 'dealer' */
            let initial_transcript = transcript.clone();

            transcript = rangeproof_domain_sep(transcript, U64::classify(n as u64), U64::classify(1u64));

            /* Create single 'party' */

            

        }

        

        /*

        let (party_awaiting_bit_challenge, bitcommitment) = new_party(bp_gens, pc_gens, value, blinding, n);

        let (dealer_awaiting_polycommitment, bit_challenge) = create_bit_challenge(dealer_awaiting_bit_commitment, bit_commitment);

        let (party_awaiting_poly_challenge, poly_commitment) = create_polycommitment(party_awaiting_bit_challenge, bit_challenge);

        let (dealer_awaiting_proofshare, poly_challenge) = create_polychallenge(dealer_awaiting_poly_commitment, poly_commitment);

        let proofshare = create_proofshare(party_awaiting_poly_challenge,poly_challenge);

        let range_proof = create_range_proof(dealer_awaiting_proofshare, proofshare);*/


    }