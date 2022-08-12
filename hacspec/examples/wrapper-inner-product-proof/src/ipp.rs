#![feature(int_log)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

use hacspec_lib::*;
use merlin::Transcript as rust_transcript;
use wrapper_hacspec_ristretto::*;
use bulletproofs::inner_product_proof::InnerProductProof as rust_ipp;

mod errors;
use errors::*;

//a, b, L_vec, R_vec
pub type InnerProductProof = rust_ipp;
pub type Transcript = rust_transcript;

// Result types for brevity
type IppRes = Result<(Transcript, InnerProductProof), u8>;
type VerScalarsResVec = Result<(Vec<Scalar>, Vec<Scalar>,Vec<Scalar>),u8>;
type VerScalarsRes = Result<(Seq<Scalar>, Seq<Scalar>, Seq<Scalar>), u8>;
type VerifyRes = Result<(), u8>;
type DecodeRes = Result<RistrettoPoint, u8>;


fn seq_to_vec_scalar(seq: Seq<Scalar>) -> Vec<Scalar>{
    let mut vec = Vec::<Scalar>::new();
    for i in 0..seq.len() {
        vec.push(seq[i]);
    }
    vec
}

fn vec_to_seq_scalar(vec: Vec<Scalar>) -> Seq<Scalar>{
    let mut seq = Seq::<Scalar>::new(vec.len());
    for i in 0..vec.len() {
        seq[i] = vec[i];
    }
    seq
}

fn seq_to_vec_point(seq: Seq<RistrettoPoint>) -> Vec<RistrettoPoint>{
    let mut vec = Vec::<RistrettoPoint>::new();
    for i in 0..seq.len() {
        vec.push(seq[i]);
    }
    vec
}
// === External Functions === //

/// Create an inner product proof.
/// Note that `G_factors`, and `H_factors` simply represent factors
/// multiplied to the `G` and `H` Seqs respectively.
pub fn create(
    mut transcript: Transcript,
    Q: RistrettoPoint,
    G_factors: Seq<Scalar>,
    H_factors: Seq<Scalar>,
    mut G: Seq<RistrettoPoint>,
    mut H: Seq<RistrettoPoint>,
    mut a: Seq<Scalar>,
    mut b: Seq<Scalar>, 
) -> IppRes {
    IppRes::Ok((transcript.clone(),
    rust_ipp::create(
        &mut transcript,
        &Q,
        &seq_to_vec_scalar(G_factors),
        &seq_to_vec_scalar(H_factors),
        seq_to_vec_point(G),
        seq_to_vec_point(H),
        seq_to_vec_scalar(a),
        seq_to_vec_scalar(b)
    )))

}


/// Returns (u^2, s, u^-2) to be used in the verification algorithm.
/// It is public as it can be faster and more convenient to simply get
/// these values and do the verification outside this module.
pub fn verification_scalars(
    ipp: InnerProductProof,
    n: usize,
    mut transcript: Transcript,
) -> VerScalarsRes {

    let (a,b,c) = rust_ipp::verification_scalars(&ipp,n,&mut transcript).or(VerScalarsResVec::Err(VERIFICATION_ERROR))?;
    VerScalarsRes::Ok((vec_to_seq_scalar(a), vec_to_seq_scalar(b), vec_to_seq_scalar(c)))

}

/// Checks if a given InnerProductProof and P have been honestly
/// constructed. Returns an error if the verification check fails.
pub fn verify(
    ipp: InnerProductProof,
    n: usize,
    mut transcript: Transcript,
    G_factors: Seq<Scalar>,
    H_factors: Seq<Scalar>,
    P: RistrettoPoint,
    Q: RistrettoPoint,
    G: Seq<RistrettoPoint>,
    H: Seq<RistrettoPoint>,
) -> VerifyRes {
    rust_ipp::verify(
        &ipp,
        n,
        &mut transcript,
        &seq_to_vec_scalar(G_factors),
        &seq_to_vec_scalar(H_factors),
        &P,
        &Q,
        &seq_to_vec_point(G),
        &seq_to_vec_point(H)
    ).or(VerifyRes::Err(VERIFICATION_ERROR))
}
