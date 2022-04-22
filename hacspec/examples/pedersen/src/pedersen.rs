use hacspec_lib::*;
use hacspec_curve25519::*;


pub fn pederson_commit(randomness: X25519SerializedScalar, secret_key: X25519SerializedScalar, message: X25519SerializedScalar) -> X25519SerializedPoint {

	let h = x25519_scalarmult(secret_key, BASE_POINT());

	let r_h = x25519_scalarmult(randomness,h);

	let a_g = x25519_scalarmult(message, BASE_POINT());

	//let commitment = x25519_addpoints(a_g, r_h);

	a_g

}
