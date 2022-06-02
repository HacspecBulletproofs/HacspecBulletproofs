#![allow(non_snake_case)]
use hacspec_lib::*;
use hacspec_ristretto::*;
use hacspec_linalg_field::*;

fn point_dot(v: Matrix, G: RistrettoPoint) -> RistrettoPoint {
	let mut acc = IDENTITY_POINT();
	let (_, entries) = v; 
	for i in 0.. entries.len() {
		acc = add(acc, mul(entries[i], G));
	}
	acc
}

//r must be random
pub fn pedersen_commit(r: Scalar, H: RistrettoPoint, a: Scalar, G: RistrettoPoint) -> RistrettoPoint {
	let rH = mul(r, H);
	let aG = mul(a, G);

	add(aG, rH)
}

pub fn vector_pedersen_commit(r: Scalar, G: RistrettoPoint, H: RistrettoPoint, v: Matrix) -> RistrettoPoint {
	let rH = mul(r,H);
	let vG = point_dot(v, G);

	add(rH, vG)
}
