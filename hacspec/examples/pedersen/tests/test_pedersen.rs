#![allow(non_snake_case)]
use curve25519_dalek::ristretto::RistrettoPoint as DalekRistrettoPoint;
use hacspec_lib::*;
use hacspec_pedersen::*;
use hacspec_ristretto::*;
use quickcheck::*;

fn quickcheck(tests: u64, helper: impl Testable) {
    QuickCheck::new()
        .tests(tests)
        .min_tests_passed(tests)
        .max_tests(10000000000)
        .quickcheck(helper);
}

#[test]
fn unit_test() {
	let x = Scalar::from_hex("00000000000000000000abcdef00000000000000000000000000000000000003");
	let H = mul(x, BASE_POINT());

	let r1: Scalar = Scalar::from_hex("0000000000000000000000000000000000000000000000000000f34000000000");
	let r2: Scalar = Scalar::from_hex("0000000000000000000000000000000000000000000000000100000000000000");
	let r3: Scalar = r1 + r2;

	let a1: Scalar = Scalar::from_hex("0000000000000000000000000000000000000000000000000b00000000000000");
	let a2: Scalar = Scalar::from_hex("0000000000000000000000000000000000000000000000658a00000000000001");
	let a3: Scalar = a1 + a2;

	let c1 = pedersen_commit(r1, H, a1, BASE_POINT());
	let c2 = pedersen_commit(r2, H, a2, BASE_POINT());
	let c_sum1 = pedersen_commit(r3, H, a3, BASE_POINT());
	let c_sum2 = add(c1, c2);

	assert!(equals(c_sum1,c_sum2));
}

#[test]
fn quickcheck_pedersen_commit() {
	fn helper(random1:u64, random2: u64, message1: u64, message2: u64, blinding_point_scalar: u64) -> TestResult{

		if blinding_point_scalar == 0 {
			return TestResult::discard();
		}
		let q = Scalar::from_literal(blinding_point_scalar.into());
		let G = BASE_POINT();
		let H = mul(q,G);

		let r1 = Scalar::from_literal(random1.into());
		let a1 = Scalar::from_literal(message1.into());

		let r2 = Scalar::from_literal(random2.into());
		let a2 = Scalar::from_literal(message2.into());

		let r3 = r1 + r2;
		let a3 = a1 + a2;
		
		let commitment1 = pedersen_commit(r1,H,a1,G);
		let commitment2 = pedersen_commit(r2,H,a2,G);

		let sum_of_commitments = add(commitment1,commitment2);

		let commitment_of_sum = pedersen_commit(r3,H,a3,G);

		println!("a1: {:?}", a1);
		println!("a2: {:?}", a2);
		println!("a3: {:?}", a3);
		println!("r1: {:?}", r1);
		println!("r2: {:?}", r2);
		println!("r3: {:?}", r3);
		println!("q: {:?}", q);
		println!("sum of commit: {:?}", sum_of_commitments);
		println!("commit of sum: {:?}", commitment_of_sum);
		println!();

		TestResult::from_bool(equals(sum_of_commitments,commitment_of_sum))

	}
	quickcheck(50, helper as fn(u64,u64,u64,u64,u64) -> TestResult);
}




