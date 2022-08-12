#![allow(non_snake_case)]
use wrapper_hacspec_ristretto::*;

// === Helper Functions === //


// === External Functions === //

//r must be random
pub fn pedersen_commit(
    r: Scalar,
    H: RistrettoPoint,
    a: Scalar,
    G: RistrettoPoint,
) -> RistrettoPoint {
    let rH = mul(r, H);
    let aG = mul(a, G);

    add(aG, rH)
}