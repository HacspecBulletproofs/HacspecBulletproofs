#![allow(non_snake_case)]
use hacspec_lib::*;
use hacspec_linalg_field as linalg;
use hacspec_linalg_field::*;
use hacspec_ristretto::*;

// === Helper Functions === //

//We need to generate better Base Points
fn point_dot(v: Matrix, G: RistrettoPoint) -> RistrettoPoint {
    let mut acc = IDENTITY_POINT();
    for i in 0..v.1.len() {
        acc = ristretto::add(acc, ristretto::mul(v.1[i], G));
    }
    acc
}

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

pub fn vector_pedersen_commit(
    r: Scalar,
    G: RistrettoPoint,
    H: RistrettoPoint,
    v: Matrix,
) -> RistrettoPoint {
    let rH = mul(r, H);
    let vG = point_dot(v, G);

    add(rH, vG)
}
