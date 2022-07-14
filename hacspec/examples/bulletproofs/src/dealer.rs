#![allow(non_snake_case)]

mod transcript;
mod errors;
mod types;
use transcript::*;
use errors::*;
use types::*;

use hacspec_lib::*;
use hacspec_merlin::*;
use hacspec_ristretto::*;
use hacspec_ipp::*;
//use hacspec_pedersen::*;

type CreateDealerRes = Result<DealerAwaitingBitCommitments,u8>;
type ReceiveBitsRes = Result<(DealerAwaitingPolyCommitments,(Scalar,Scalar)),u8>;
type ReceivePolyCommitmentsRes = Result<(DealerAwaitingProofShares, Scalar), u8>;
type ReceiveSharesRes = Result<RangeProof, u8>;

pub type DealerAwaitingBitCommitments = (
    /*bp_gens:*/ BulletproofGens,
    /*pc_gens:*/ PedersenGens,
    /*transcript:*/ Transcript,
    /*Initial transcript:*/ Transcript,
    /*n:*/ usize,
    /*m:*/ usize
);

pub type DealerAwaitingPolyCommitments = (
    /*n:*/ usize,
    /*m:*/ usize,
    /*transcript:*/ Transcript,
    /*initial_transcript:*/ Transcript,
    /*bp_gens:*/ BulletproofGens,
    /*pc_gens:*/ PedersenGens,
    /*bit_challenge:*/ (Scalar,Scalar),
    /*bit_commitments:*/ Seq<(RistrettoPointEncoded,RistrettoPoint,RistrettoPoint)>,
    /* Aggregated commitment to the parties' bits*/
    /*A:*/ RistrettoPoint,
    /* Aggregated commitment to the parties' bit blindings */
    /*S:*/ RistrettoPoint
);

pub type DealerAwaitingProofShares = (
    /*n:*/ usize,
    /*m:*/ usize,
    /*transcript:*/ Transcript,
    /*initial_transcript:*/ Transcript,
    /*bp_gens:*/ BulletproofGens,
    /*pc_gens:*/ PedersenGens,
    /*bit_challenge:*/ (Scalar, Scalar),
    /*bit_commitments:*/ Seq<(RistrettoPointEncoded, RistrettoPoint, RistrettoPoint)>,
    /*poly_challenge:*/ Scalar,
    /*poly_commitments:*/ Seq<(RistrettoPoint,RistrettoPoint)>,
    /*A:*/ RistrettoPoint,
    /*S:*/ RistrettoPoint,
    /*T_1:*/ RistrettoPoint,
    /*T_2:*/ RistrettoPoint
);

pub type RangeProof = (
    /* Commitment to the bits of the value */
    /*A:*/ RistrettoPointEncoded,
    /* Commitment to the blinding factors */
    /*S:*/ RistrettoPointEncoded,
    /* Commitment to the \\(t_1\\) coefficient of \\( t(x) \\) */
    /*T_1:*/ RistrettoPointEncoded,
    /* Commitment to the \\(t_2\\) coefficient of \\( t(x) \\)*/
    /*T_2:*/ RistrettoPointEncoded,
    /* Evaluation of the polynomial \\(t(x)\\) at the challenge point \\(x\\)*/
    /*t_x:*/ Scalar,
    /* Blinding factor for the synthetic commitment to \\(t(x)\\)*/
    /*t_x_blinding:*/ Scalar,
    /* Blinding factor for the synthetic commitment to the inner-product arguments*/
    /*e_blinding:*/ Scalar,
    /* Proof data for the inner-product argument.*/
    /*ipp_proof:*/ InnerProductProof
);

pub fn create_dealer(        
    bp_gens: BulletproofGens,
    pc_gens: PedersenGens,
    mut transcript: Transcript,
    n: usize,
    m: usize) -> CreateDealerRes {

    let (party_capacity, gens_capacity, g_vec, h_vec) = bp_gens;
    #[allow(unused)]
    let mut res = CreateDealerRes::Err(0u8);

    if !(n == 8 || n == 16 || n == 32 || n == 64) {
        res = CreateDealerRes::Err(INVALID_BIT_SIZE);
    }
    else {if !m.is_power_of_two() {
        res = CreateDealerRes::Err(INVALID_AGGREGATION);
    }
    else {if gens_capacity < n {
        res = CreateDealerRes::Err(INVALID_GENERATORS_LENGTH);
    }
    else {if party_capacity < m {
        res = CreateDealerRes::Err(INVALID_GENERATORS_LENGTH);
    }
    else {

        let initial_transcript = transcript;

        transcript = rangeproof_domain_sep(transcript, U64::classify(n as u64), U64::classify(m as u64));

        res = CreateDealerRes::Ok(((party_capacity,gens_capacity,g_vec,h_vec), pc_gens, transcript, initial_transcript, n, m));
    }}}}
    res
}

pub fn receive_bit_commitments(
    dealer: DealerAwaitingBitCommitments, 
    bit_commitments: Seq<(RistrettoPointEncoded,RistrettoPoint,RistrettoPoint)>
    ) -> ReceiveBitsRes {
    #[allow(unused)]
    let mut res = ReceiveBitsRes::Err(0u8);

    let (bp_gens,pc_gens, mut transcript,initial_transcript,n,m) = dealer;

    if m != bit_commitments.len() {
        res = ReceiveBitsRes::Err(WRONG_NUMBER_OF_BIT_COMMITMENTS);
    }
    else {
        let number_of_commits = bit_commitments.len();

        let mut A = IDENTITY_POINT();
        let mut S = IDENTITY_POINT();
        // Commit each V_i individually
        for i in 0..number_of_commits {
            let (V_i, A_i, S_i) = bit_commitments[i];
            transcript = append_point(transcript, byte_seq!(86u8), V_i);
            A = add(A_i,A);
            S = add(S_i,S);
        }

        transcript = append_point(transcript, byte_seq!(65u8), encode(A));
        transcript = append_point(transcript, byte_seq!(83u8), encode(S));

        let (temp_transcript, y) = challenge_scalar(transcript, byte_seq!(121u8));
        let (new_transcript, z) = challenge_scalar(temp_transcript, byte_seq!(122u8));

        let new_dealer = (n,m,new_transcript,initial_transcript,bp_gens,pc_gens,(y,z),bit_commitments,A,S);
        res = ReceiveBitsRes::Ok((new_dealer,(y,z)));
    }
    res
}

pub fn receive_poly_commitments(
    dealer: DealerAwaitingPolyCommitments,
    poly_commitments: Seq<(RistrettoPoint,RistrettoPoint)>,
) -> ReceivePolyCommitmentsRes {

    #[allow(unused)]
    let mut res = ReceivePolyCommitmentsRes::Err(0u8);

    let (n, m, transcript, initial_transcript, bp_gens, pc_gens, bit_challenge, bit_commitments, A, S) = dealer;

    if m != poly_commitments.len() {
        res = ReceivePolyCommitmentsRes::Err(WRONG_NUMBER_OF_POLY_COMMITMENTS);
    }
    else {

        let mut T_1 = IDENTITY_POINT();
        let mut T_2 = IDENTITY_POINT();

        for i in 0..poly_commitments.len() {
            let (T_1_j, T_2_j) = poly_commitments[i];
            T_1 = add(T_1, T_1_j);
            T_2 = add(T_2, T_2_j);
        }

        let transcript_temp_1 = append_point(transcript, byte_seq!(84u8, 95u8, 49u8), encode(T_1));
        let transcript_temp_2 = append_point(transcript_temp_1, byte_seq!(84u8, 95u8, 50u8), encode(T_2));

        let (new_transcript, poly_challenge) = challenge_scalar(transcript_temp_2, byte_seq!(120u8));

        let new_dealer = (
            n,
            m,
            new_transcript,
            initial_transcript,
            bp_gens,
            pc_gens,
            bit_challenge,
            bit_commitments,
            poly_challenge,
            poly_commitments,
            A,
            S,
            T_1,
            T_2
            );
            res = ReceivePolyCommitmentsRes::Ok((new_dealer, poly_challenge));
        
    }
    res
}


pub fn receive_shares(dealer: DealerAwaitingProofShares, 
                      t_xs: Seq<Scalar>, 
                      t_x_blindings: Seq<Scalar>, 
                      e_blindings: Seq<Scalar>, 
                      l_vecs: Seq<Seq<Scalar>>, 
                      r_vecs: Seq<Seq<Scalar>>) 
                      -> ReceiveSharesRes {

    #[allow(unused)]
    let mut res = ReceiveSharesRes::Err(0u8);

    let (n,m, mut transcript,_,(party_capacity, gens_capacity,g_vec,h_vec),pc_gens,bit_challenge,_,_,_,A,S,T_1,T_2) = dealer;

    if m != t_xs.len() { //all sequence inputs are the same length
        res = ReceiveSharesRes::Err(WRONG_NUMBER_OF_PROOF_SHARES);
    }
    else {

        // Validate lengths for each share
        let mut has_bad_shares = false;
        for j in 0..m {
            has_bad_shares = has_bad_shares || check_share_size(l_vecs[j].clone(), r_vecs[j].clone(), n, party_capacity, gens_capacity,j);
        }

        if has_bad_shares {
            res = ReceiveSharesRes::Err(MALFORMED_PROOF_SHARES);
        }
        else {
            let mut t_x = Scalar::from_literal(0u128);
            let mut t_x_blinding = Scalar::from_literal(0u128);
            let mut e_blinding = Scalar::from_literal(0u128);

            for i in 0..m {
                let t_x_i = t_xs[i];
                let t_x_blinding_i = t_x_blindings[i];
                let e_blinding_i = e_blindings[i];
                t_x = t_x + t_x_i;
                t_x_blinding = t_x_blinding + t_x_blinding_i;
                e_blinding = e_blinding + e_blinding_i;
            }
            let t_x_label = byte_seq!(116u8, 95u8, 120u8); /* "t_x" */
            let t_x_blinding_label = byte_seq!(116u8, 95u8, 120u8, 95u8, 98u8, 108u8, 105u8, 110u8, 100u8, 105u8, 110u8, 103u8); /* "t_x_blinding" */
            let e_blinding_label = byte_seq!(101u8, 95u8, 98u8, 108u8, 105u8, 110u8, 100u8, 105u8, 110u8, 103u8); /* "e_blinding" */
            transcript = append_scalar(transcript, t_x_label, t_x);
            transcript = append_scalar(transcript, t_x_blinding_label, t_x_blinding);
            transcript = append_scalar(transcript,e_blinding_label, e_blinding);
            
            // Get a challenge value to combine statements for the IPP
            let (new_transcript, w) = challenge_scalar(transcript, byte_seq!(119u8));
            let (base_point, _) = pc_gens;
            let Q = mul(w, base_point);
            
            let mut G_factors = Seq::<Scalar>::new(n*m);
            let mut H_factors = Seq::<Scalar>::new(n*m);
            let (_, bit_challenge_y) = bit_challenge;
            let one = Scalar::from_literal(1u128);
            let y_inv = bit_challenge_y.inv();

            for i in 0..n*m {
                G_factors[i] = one;
                H_factors[i] = y_inv^(Scalar::from_literal(i as u128));
            }

            let mut l_vec = Seq::<Scalar>::new(n * t_xs.len());
            let mut r_vec = Seq::<Scalar>::new(n * t_xs.len());

            for i in 0..t_xs.len() {
                let l_vec_i = l_vecs[i].clone();
                let r_vec_i = r_vecs[i].clone();
                for j in 0..l_vec_i.len() {
                    l_vec[i*l_vecs.len() + j] = l_vec_i[j];
                    r_vec[i*l_vecs.len() + j] = r_vec_i[j];
                }
            }

            // Here we begin using g_vec and h_vec which were unpacked from dealer

            let mut G = Seq::<RistrettoPoint>::new(n*m);
            let mut H = Seq::<RistrettoPoint>::new(n*m);

            for i in 0..m {
                let g_vec_i = g_vec[i].clone();
                let h_vec_i = h_vec[i].clone();
                for j in 0..n {
                    G[m*i + j] = g_vec_i[j];
                    H[m*i + j] = h_vec_i[j];

                }
            }

            let (_, ipp) = create(new_transcript,Q,G_factors,H_factors,G,H, l_vec,r_vec)?;

            let rangeproof = (encode(A),encode(S),encode(T_1),encode(T_2),t_x,t_x_blinding,e_blinding,ipp);

            res = ReceiveSharesRes::Ok(rangeproof);

    }}
    res
}

fn check_share_size(l_vec: Seq<Scalar>, r_vec: Seq<Scalar>, expected_n: usize, party_capacity: usize, gens_capacity: usize, j: usize) -> bool {
    l_vec.len() != expected_n 
    || r_vec.len() != expected_n 
    || expected_n > gens_capacity
    || j >= party_capacity
}