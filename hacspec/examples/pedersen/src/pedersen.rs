use hacspec_lib::*;
use hacspec_secp256k1::*;


pub fn pederson_commit(randomness: Secp256k1Scalar, secret_key: Secp256k1Scalar, message: Secp256k1Scalar) -> Affine {

	let h = scalar_multiplication(secret_key, BASE_POINT());

	let r_h = scalar_multiplication(randomness,h);

	let a_g = scalar_multiplication(message, BASE_POINT());

	let commitment = add_points(a_g, r_h);

	commitment

}
