//Todo remove decode_hex
#![allow(non_snake_case)]
extern crate quickcheck;
extern crate hex;

use hacspec_lib::*;
use hacspec_ristretto::*;
use curve25519_dalek::ristretto::RistrettoPoint as DalekRistrettoPoint;
use curve25519_dalek::ristretto::CompressedRistretto as DalekRistrettoPointEncoded;
use quickcheck::*;

// === Helper Functions

fn quickcheck(tests: u64, helper: impl Testable) {
    QuickCheck::new()
        .tests(tests)
        .min_tests_passed(tests)
        .max_tests(100000000)
        .quickcheck(helper);
}

fn cmp_points(p: RistrettoPoint, q: DalekRistrettoPoint) -> bool {
	let p_enc = encode(p);
	let p_bytes = p_enc.to_le_bytes();
	let p_native = p_bytes.to_native();
	let p_slice = p_native.as_slice();

	let q_enc = q.compress();
	let q_slice = q_enc.to_bytes();

	q_slice == p_slice
}

fn is_bigger_than_p(n: Vec<u8>) -> bool {
    let mut n_ = n;
    let p = hex::decode("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed").unwrap();
    n_.reverse();
    n_.cmp(&p) == Ordering::Greater
}

// === Tests ===

#[test]
fn test_dalek_decode_encode() {
    fn helper(mut v: Vec<u8>) -> TestResult {
        if v.len() < 32 {
            return TestResult::discard();
        }
        v.truncate(32);

        if is_bigger_than_p(v.clone()) {
            return TestResult::discard();
        }
        let v = v.as_slice();

        let hac_enc = RistrettoPointEncoded::from_public_slice(v);
        let dal_enc = DalekRistrettoPointEncoded::from_slice(&v);

        let hac_dec = decode(hac_enc);
        let dal_dec = dal_enc.decompress();

        if hac_dec.is_err() && dal_dec.is_none() {
            return TestResult::discard();
        }

        let hac_dec = hac_dec.unwrap();
        let dal_dec = dal_dec.unwrap();

        TestResult::from_bool(cmp_points(hac_dec, dal_dec))
    }
    quickcheck(10, helper as fn(Vec<u8>) -> TestResult)
}

#[test]
fn test_dalek_point_addition() {
    fn helper(mut v: Vec<u8>, mut u: Vec<u8>) -> TestResult {
        if v.len() < 32 ||  u.len() < 32 {
            return TestResult::discard();
        }
        v.truncate(32);
        u.truncate(32);

        if is_bigger_than_p(v.clone()) || is_bigger_than_p(u.clone()) {
            return TestResult::discard();
        }

        let v = v.as_slice();
        let u = u.as_slice();

        let hac_enc_v = RistrettoPointEncoded::from_public_slice(v);
        let hac_enc_u = RistrettoPointEncoded::from_public_slice(u);

        let dal_enc_v = DalekRistrettoPointEncoded::from_slice(&v);
        let dal_enc_u = DalekRistrettoPointEncoded::from_slice(&u);

        let hac_dec_res_v = decode(hac_enc_v);
        let hac_dec_res_u = decode(hac_enc_u);

        let dal_dec_res_v = dal_enc_v.decompress();
        let dal_dec_res_u = dal_enc_u.decompress();

        if hac_dec_res_v.is_err() && dal_dec_res_v.is_none() {
            return TestResult::discard();
        }
        if hac_dec_res_u.is_err() && dal_dec_res_u.is_none() {
            return TestResult::discard();
        }

        let hac_dec_v = hac_dec_res_v.unwrap();
        let hac_dec_u = hac_dec_res_u.unwrap();
        let dal_dec_v = dal_dec_res_v.unwrap();
        let dal_dec_u = dal_dec_res_u.unwrap();

        let hac_add = add(hac_dec_v, hac_dec_u);
        let hac_sub = sub(hac_dec_v, hac_dec_u);

        let dal_add = dal_dec_v + dal_dec_u;
        let dal_sub = dal_dec_v - dal_dec_u;

        TestResult::from_bool(cmp_points(hac_add, dal_add) && cmp_points(hac_sub, dal_sub))
    }
    quickcheck(10, helper as fn(Vec<u8>, Vec<u8>) -> TestResult)
}

#[test]
fn test_dalek_scalar_multiplication() {
	fn helper(mut v: Vec<u8>, x: u128) -> TestResult {
		if v.len() < 32 {
			return TestResult::discard();
		}
		v.truncate(32);

		if is_bigger_than_p(v.clone()) {
			return TestResult::discard();
		}

		let v = v.as_slice();

		let hac_enc = RistrettoPointEncoded::from_public_slice(v);
		let dal_enc = DalekRistrettoPointEncoded::from_slice(&v);

		let hac_dec = decode(hac_enc);
		let dal_dec = dal_enc.decompress();

		if hac_dec.is_err() && dal_dec.is_none() {
			return TestResult::discard();
		}

		let hac_dec = hac_dec.unwrap();
		let dal_dec = dal_dec.unwrap();

		let hac_scal = mul(flit(x), hac_dec);
		let dal_scal = curve25519_dalek::scalar::Scalar::from(x) * dal_dec;

		TestResult::from_bool(cmp_points(hac_scal, dal_scal))
	}
	quickcheck(10, helper as fn(Vec<u8>, u128) -> TestResult)
}

#[test]
fn test_dalek_point_negation() {
    fn helper(mut v: Vec<u8>) -> TestResult {
		if v.len() < 32 {
			return TestResult::discard();
		}
		v.truncate(32);

		if is_bigger_than_p(v.clone()) {
			return TestResult::discard();
		}

		let v = v.as_slice();

		let hac_enc = RistrettoPointEncoded::from_public_slice(v);
		let dal_enc = DalekRistrettoPointEncoded::from_slice(&v);

		let hac_dec = decode(hac_enc);
		let dal_dec = dal_enc.decompress();

		if hac_dec.is_err() && dal_dec.is_none() {
			return TestResult::discard();
		}

		let hac_dec = hac_dec.unwrap();
		let dal_dec = dal_dec.unwrap();

		let hac_scal = neg(hac_dec);
		let dal_scal = dal_dec.neg();

		TestResult::from_bool(cmp_points(hac_scal, dal_scal))
	}
	quickcheck(1000, helper as fn(Vec<u8>) -> TestResult)
}
