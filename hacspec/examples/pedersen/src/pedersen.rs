use hacspec_lib::*;
use hacspec_ristretto::*;
use hacspec_linalg_field::*;
use hacspec_ristretto as ristretto;
use hacspec_linalg_field as linalg;

//We need to generate better Base Points
fn point_dot(v: Matrix, p: RistrettoPoint) -> RistrettoPoint {
	let mut acc = IDENTITY_POINT();
	for i in 0..v.1.len() {
		acc = ristretto::add(acc, ristretto::mul(v.1[i], BASE_POINT()));
	}
	acc
}

//r must be random
pub fn pederson_commit(r: FieldElement, H: RistrettoPoint, a: FieldElement) -> RistrettoPoint {
	let rH = ristretto::mul(r,H);
	let aG = ristretto::mul(a, BASE_POINT());

	ristretto::add(aG, rH)
}

pub fn vector_pedersen_commit(r: FieldElement, H: RistrettoPoint, v: Matrix) -> RistrettoPoint {
	let rH = ristretto::mul(r,H);
	let vG = point_dot(v, BASE_POINT());

	ristretto::add(rH, vG)
}
