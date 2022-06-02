#![allow(non_snake_case)]
//use hacspec_lib::prelude::*;
use hacspec_pedersen::*;
use hacspec_ristretto::*;
use quickcheck::*;

fn quickcheck(helper: impl Testable) {
    let tests = 10;
    QuickCheck::new()
        .tests(tests)
        .min_tests_passed(tests)
        .max_tests(100000000)
        .quickcheck(helper);
}

#[test]
fn unit_test() {
	let x = Scalar::from_hex("00000000000000000000000090000000000000000000000000e0000000000001");
	let H = mul(x, BASE_POINT());

	let r1: Scalar = Scalar::from_hex("0006730000000000000000000100000000000000000000000000000000000001");
	let r2: Scalar = Scalar::from_hex("00000abcd0000000000000000200000000000000000000000000000000000001");
	let r3: Scalar = r1 + r2;

	let a1: Scalar = Scalar::from_hex("0000000000000000000000000000400000000000000000000000000000000001");
	let a2: Scalar = Scalar::from_hex("0000000000000000000000000000000000000000000000000000000000000001");
	let a3: Scalar = a1 + a2;

	let c1 = pedersen_commit(r1,BASE_POINT(),H,a1);
	let c2 = pedersen_commit(r2,BASE_POINT(),H,a2);
	let c_sum1 = pedersen_commit(r3,BASE_POINT(),H,a3);
	let c_sum2 = add(c1, c2);

	assert!(equals(c_sum1,c_sum2));
}
//#[test]
//fn test_1() {	
//	fn helper(q: String, r1: String, r2: String, m1: String, m2: String) -> TestResult{
//
//		let secret_hex = q.as_bytes().to_hex();
//		let random1_hex = r1.as_bytes().to_hex();
//		let random2_hex = r2.as_bytes().to_hex();
//		let message1_hex = m1.as_bytes().to_hex();
//		let message2_hex = m2.as_bytes().to_hex();	
//
//		if secret_hex.chars().count() < 64{
//			TestResult::discard()
//		}
//		else if random1_hex.chars().count() < 64 {
//			TestResult::discard()
//		}
//		else if random2_hex.chars().count() < 64 {
//			TestResult::discard()
//		}
//		else if message1_hex.chars().count() < 64 {
//			TestResult::discard()
//		}
//		else if message2_hex.chars().count() < 64 {
//			TestResult::discard()
//		}
//		else{
//
//			let (secret,_) = secret_hex.split_at(64);
//			let (rand1,_) = random1_hex.split_at(64);
//			let (rand2,_) = random2_hex.split_at(64);
//			let (mess1,_) = message1_hex.split_at(64);
//			let (mess2,_) = message2_hex.split_at(64);
//
//			let secret_key = X25519SerializedScalar::from_hex(&secret);
//			
//			let randomness1: X25519SerializedScalar = X25519SerializedScalar::from_hex(&rand1);
//			let randomness2: X25519SerializedScalar = X25519SerializedScalar::from_hex(&rand2);
//			let randomness3: X25519SerializedScalar = randomness1 + randomness2;
//
//			let message1: X25519SerializedScalar = X25519SerializedScalar::from_hex(&mess1);
//			let message2: X25519SerializedScalar = X25519SerializedScalar::from_hex(&mess2);
//			let message3: X25519SerializedScalar = message1 + message2;
//
//			let commitment1 = pederson_commit(randomness1,secret_key,message1);
//			let commitment2 = pederson_commit(randomness2,secret_key,message2);
//			let commitment_of_sum = pederson_commit(randomness3,secret_key,message3);
//			let sum_of_commitment = x25519_addpoints(commitment1, commitment2);
//
//			TestResult::from_bool(commitment_of_sum.to_hex() == sum_of_commitment.to_hex())
//		}
//	}
//	QuickCheck::new()
//		.tests(5)
//		.max_tests(10000000000000)
//		.quickcheck(helper as fn(String,String,String,String,String) -> TestResult);
//}
//
////#[test]
//fn test_2() {
//	fn helper(q: String, r1: String, r2: String, m1: String, m2: String) -> TestResult{
//
//		let secret_hex = q.as_bytes().to_hex();
//		let random1_hex = r1.as_bytes().to_hex();
//		let random2_hex = r2.as_bytes().to_hex();
//		let message1_hex = m1.as_bytes().to_hex();
//		let message2_hex = m2.as_bytes().to_hex();	
//
//		if secret_hex.chars().count() < 64{
//			TestResult::discard()
//		}
//		else if random1_hex.chars().count() < 64 {
//			TestResult::discard()
//		}
//		else if random2_hex.chars().count() < 64 {
//			TestResult::discard()
//		}
//		else if message1_hex.chars().count() < 64 {
//			TestResult::discard()
//		}
//		else if message2_hex.chars().count() < 64 {
//			TestResult::discard()
//		}
//		else{
//
//			let (secret,_) = secret_hex.split_at(64);
//			let (rand1,_) = random1_hex.split_at(64);
//			let (rand2,_) = random2_hex.split_at(64);
//			let (mess1,_) = message1_hex.split_at(64);
//			let (mess2,_) = message2_hex.split_at(64);
//
//			let secret_key = X25519SerializedScalar::from_hex(&secret);
//			
//			let randomness1: X25519SerializedScalar = X25519SerializedScalar::from_hex(&rand1);
//			let randomness2: X25519SerializedScalar = X25519SerializedScalar::from_hex(&rand2);
//			let randomness3: X25519SerializedScalar = randomness1;
//
//			let message1: X25519SerializedScalar = X25519SerializedScalar::from_hex(&mess1);
//			let message2: X25519SerializedScalar = X25519SerializedScalar::from_hex(&mess2);
//			let message3: X25519SerializedScalar = message1 + message2;
//
//			let commitment1 = pederson_commit(randomness1,secret_key,message1);
//			let commitment2 = pederson_commit(randomness2,secret_key,message2);
//			let commitment_of_sum = pederson_commit(randomness3,secret_key,message3);
//			let sum_of_commitment = x25519_addpoints(commitment1, commitment2);
//
//			TestResult::from_bool(commitment_of_sum.to_hex() != sum_of_commitment.to_hex())
//		}
//	}
//	QuickCheck::new()
//	.tests(5)	
//	.max_tests(10000000000000)
//	.quickcheck(helper as fn(String,String,String,String,String) -> TestResult);
//}




