use hacspec_lib::*;
use hacspec_ristretto::*;
use hacspec_linalg_field::*;
use hacspec_ristretto as ristretto;
use hacspec_linalg_field as linalg;

//We need to generate better Base Points
fn point_dot(v: Matrix, p: RistrettoPoint, G: RistrettoPoint) -> RistrettoPoint {
	let mut acc = IDENTITY_POINT();
	for i in 0..v.1.len() {
		acc = ristretto::add(acc, ristretto::mul(v.1[i], G));
	}
	acc
}

//r must be random
pub fn pedersen_commit(r: Scalar, G: RistrettoPoint, H: RistrettoPoint, a: Scalar) -> RistrettoPoint {
	let rH = ristretto::mul(r,H);
	let aG = ristretto::mul(a, G);

	ristretto::add(aG, rH)
}

pub fn vector_pedersen_commit(r: Scalar, G: RistrettoPoint, H: RistrettoPoint, v: Matrix) -> RistrettoPoint {
	let rH = ristretto::mul(r,H);
	let vG = point_dot(v, G, G);

	ristretto::add(rH, vG)
}
