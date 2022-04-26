#![allow(non_snake_case)]
extern crate quickcheck;

use hacspec_lib::*;
use curve25519_dalek::ristretto::*;
use hacspec_ristretto::*;
use quickcheck::*;
use std::convert::TryInto;


fn quickcheck(helper: impl Testable) {
    let tests = 1000;
    QuickCheck::new()
        .tests(tests)
        .min_tests_passed(tests)
        .max_tests(100000000)
        .quickcheck(helper);
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[test]
fn unit_test_pos_SQRT_RATIO() {
    let u = FieldElement::from_literal(8); //2*2*2
    let v = FieldElement::from_literal(2);
    let (was_SQRT,ratio) = SQRT_RATIO_M1(u,v);
    let expectedRes = FieldElement::from_literal(2);
    println!("condition: {}", was_SQRT);
    assert_eq!(ratio,expectedRes);

}

#[test]
fn unit_test_neg_SQRT_RATIO() {
    let u = FieldElement::from_literal(15);
    let v = FieldElement::from_literal(2);
    let (was_SQRT,ratio) = SQRT_RATIO_M1(u,v);
    println!("condition: {}", was_SQRT);
    assert!(!was_SQRT);
}

#[test]
fn unit_test_big_neg_decode() {
    let hexs = Seq::<&str>::from_vec(vec![
        //Non-canonical field encodings.
        "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
        "f3ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
        "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",

        //Negative field elements.
        "0100000000000000000000000000000000000000000000000000000000000000",
        "01ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
        "ed57ffd8c914fb201471d1c3d245ce3c746fcbe63a3679d51b6a516ebebe0e20",
        "c34c4e1826e5d403b78e246e88aa051c36ccf0aafebffe137d148a2bf9104562",
        "c940e5a4404157cfb1628b108db051a8d439e1a421394ec4ebccb9ec92a8ac78",
        "47cfc5497c53dc8e61c91d17fd626ffb1c49e2bca94eed052281b510b1117a24",
        "f1c6165d33367351b0da8f6e4511010c68174a03b6581212c71c0e1d026c3c72",
        "87260f7a2f12495118360f02c26a470f450dadf34a413d21042b43b9d93e1309",

        //Non-square x^2.
        "26948d35ca62e643e26a83177332e6b6afeb9d08e4268b650f1f5bbd8d81d371",
        "4eac077a713c57b4f4397629a4145982c661f48044dd3f96427d40b147d9742f",
        "de6a7b00deadc788eb6b6c8d20c0ae96c2f2019078fa604fee5b87d6e989ad7b",
        "bcab477be20861e01e4a0e295284146a510150d9817763caf1a6f4b422d67042",
        "2a292df7e32cababbd9de088d1d1abec9fc0440f637ed2fba145094dc14bea08",
        "f4a9e534fc0d216c44b218fa0c42d99635a0127ee2e53c712f70609649fdff22",
        "8268436f8c4126196cf64b3c7ddbda90746a378625f9813dd9b8457077256731",
        "2810e5cbc2cc4d4eece54f61c6f69758e289aa7ab440b3cbeaa21995c2f4232b",

        //Negative xy value.
        "3eb858e78f5a7254d8c9731174a94f76755fd3941c0ac93735c07ba14579630e",
        "a45fdc55c76448c049a1ab33f17023edfb2be3581e9c7aade8a6125215e04220",
        "d483fe813c6ba647ebbfd3ec41adca1c6130c2beeee9d9bf065c8d151c5f396e",
        "8a2e1d30050198c65a54483123960ccc38aef6848e1ec8f5f780e8523769ba32",
        "32888462f8b486c68ad7dd9610be5192bbeaf3b443951ac1a8118419d9fa097b",
        "227142501b9d4355ccba290404bde41575b037693cef1f438c47f8fbf35d1165",
        "5c37cc491da847cfeb9281d407efc41e15144c876e0170b499a96a22ed31e01e",
        "445425117cb8c90edcbc7c1cc0e74f747f2c1efa5630a967c64f287792a48a4b",

        //s = -1, which causes y = 0.
        "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f"
    ]);

    hexs.iter().for_each(|x| {println!("{}",x); assert!(decode(EncodedPoint::from_hex(x)).is_err())});
}

#[test]
fn ristretto_vs_hacspec() {
    let hex = "8a2e1d30050198c65a54483123960ccc38aef6848e1ec8f5f780e8523769ba32";
    let s = decode_hex(hex).unwrap();
    println!("{:?}", s);

    let hacspec_encoded_point = EncodedPoint::from_hex(hex);
    let rust_encoded_point = CompressedRistretto::from_slice(&s);
    let decoded_hacspec = decode(hacspec_encoded_point);
    let decoded_rust = rust_encoded_point.decompress();
    assert!(decoded_hacspec.is_err() && decoded_rust.is_none());

}

fn is_bigger_than_p (n: Vec<u8>) -> bool {
    let mut n_ = n;
    let mut p = decode_hex("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed").unwrap();
    n_.reverse();
    n_.cmp(&p) == Ordering::Greater
}

#[test]
fn quickcheck_is_negative() {

    fn helper(mut n: Vec<u8>) -> TestResult {
        if n.len() < 32 {
            return TestResult::discard()
        }
        n.truncate(32);

        if is_bigger_than_p(n.clone()) {
            return TestResult::discard()
        }

        let n_hacspec = hex::encode(n.clone());
        let n_rust = n.as_slice();


        let rust_element = RistrettoPoint::create_field_element(n_rust.try_into().unwrap());
        let hacspec_element = FieldElement::from_le_bytes(n_rust);

        let hacspec_negative = IS_NEGATIVE(hacspec_element);
        let rust_negative = rust_element.is_negative();

        println!("Rust: {:?}", rust_element);

        println!("Hacspec: {:?}", hacspec_element);

        
        TestResult::from_bool(hacspec_negative == rust_negative.into())
    }
    quickcheck(helper as fn(Vec<u8>) -> TestResult)
}


#[test]
fn quickcheck_SQRT_RATIO_M1() {

    fn helper(mut n: Vec<u8>) -> TestResult {
        if n.len() < 32 {
            return TestResult::discard()
        }
        n.truncate(32);

        if is_bigger_than_p(n.clone()) {
            return TestResult::discard()
        }

        let n = n.as_slice();

        
        let rust_element = RistrettoPoint::create_field_element(n.try_into().unwrap());

        let hacspec_element = FieldElement::from_le_bytes(n);

        let (was_square_hacspec, ratio_hacspec) = SQRT_RATIO_M1(flit(1),hacspec_element);
        let (was_square_rust, ratio_rust) = rust_element.invsqrt();

        let hacspec_ratio_bytes = ratio_hacspec.to_le_bytes();
        let hacspec_ratio_slice = hacspec_ratio_bytes.as_slice();

        let rust_ratio_bytes = ratio_rust.to_bytes();

        
        TestResult::from_bool(was_square_hacspec == was_square_rust.into() && rust_ratio_bytes == hacspec_ratio_slice)
    }
    quickcheck(helper as fn(Vec<u8>) -> TestResult)
}

#[test]
fn quickcheck_neg() {

    fn helper(mut n: Vec<u8>) -> TestResult {
        if n.len() < 32 {
            return TestResult::discard()
        }
        n.truncate(32);

        if is_bigger_than_p(n.clone()) {
            return TestResult::discard()
        }

        let n = n.as_slice();

        
        let rust_element = RistrettoPoint::create_field_element(n.try_into().unwrap());

        let hacspec_element = FieldElement::from_le_bytes(n);

        let inverse_hacspec = neg(hacspec_element);
        let inverse_rust = &(-&rust_element);

        let hacspec_inverse_bytes = inverse_hacspec.to_le_bytes();
        let hacspec_inverse_slice = hacspec_inverse_bytes.as_slice();
        let rust_inverse_bytes = inverse_rust.to_bytes();
        
        TestResult::from_bool(rust_inverse_bytes == hacspec_inverse_slice)
    }
    quickcheck(helper as fn(Vec<u8>) -> TestResult)
}

#[test]
fn quickcheck_invert() {

    fn helper(mut n: Vec<u8>) -> TestResult {
        if n.len() < 32 {
            return TestResult::discard()
        }
        n.truncate(32);

        if is_bigger_than_p(n.clone()) {
            return TestResult::discard()
        }

        let n = n.as_slice();

        
        let rust_element = RistrettoPoint::create_field_element(n.try_into().unwrap());

        let hacspec_element = FieldElement::from_le_bytes(n);

        let inverse_hacspec = invert(hacspec_element);
        let inverse_rust = rust_element.invert();

        let hacspec_inverse_bytes = inverse_hacspec.to_le_bytes();
        let hacspec_inverse_slice = hacspec_inverse_bytes.as_slice();
        let rust_inverse_bytes = inverse_rust.to_bytes();
        
        TestResult::from_bool(rust_inverse_bytes == hacspec_inverse_slice)
    }
    quickcheck(helper as fn(Vec<u8>) -> TestResult)
}

#[test]
fn quickcheck_decode() { //Note: this test only checks if our decode function fails if and only if the other fails as well.

    fn helper(mut n: Vec<u8>) -> TestResult {
        if n.len() < 32 {
            return TestResult::discard()
        }
        n.truncate(32);

        if is_bigger_than_p(n.clone()) {
            return TestResult::discard()
        }

        let n = n.as_slice();

        let hacspec_encoded_point = EncodedPoint::from_public_slice(n);

        let rust_encoded_point = CompressedRistretto::from_slice(&n);

        let h_decoded = decode(hacspec_encoded_point);

        let r_decoded = rust_encoded_point.decompress();

        if h_decoded.is_err() && r_decoded.is_none() {
            return TestResult::from_bool(true)
        }

        let hacspec_decoded = h_decoded.unwrap();
        let rust_decoded = r_decoded.unwrap();
        
        TestResult::from_bool(true)
    }
    quickcheck(helper as fn(Vec<u8>) -> TestResult)
}


#[test]
fn quickcheck_decode_encode() { 

    fn helper(mut n: Vec<u8>) -> TestResult {
        if n.len() < 32 {
            return TestResult::discard()
        }
        n.truncate(32);

        if is_bigger_than_p(n.clone()) {
            return TestResult::discard()
        }

        let n = n.as_slice();

        let hacspec_encoded_point = EncodedPoint::from_public_slice(n);

        let rust_encoded_point = CompressedRistretto::from_slice(&n);

        let h_decoded = decode(hacspec_encoded_point);

        let r_decoded = rust_encoded_point.decompress();

        if h_decoded.is_err() && r_decoded.is_none() {
            return TestResult::discard()
        }

        let hacspec_decoded = h_decoded.unwrap();
        let rust_decoded = r_decoded.unwrap();

        let hacspec_reencoded = encode(hacspec_decoded);

        let rust_reencoded = rust_decoded.compress();

        let hacspec_bytes = hacspec_reencoded.to_le_bytes();
        let hacspec_native = hacspec_bytes.to_native();
        let hacspec_slice = hacspec_native.as_slice();

        let rust_slice = rust_reencoded.to_bytes();

        
        TestResult::from_bool(rust_slice == hacspec_slice)
    }
    quickcheck(helper as fn(Vec<u8>) -> TestResult)
}

#[test]

#[test]
fn unit_test_point_addition() {

}

#[test]
fn unit_test_point_negation() {
    
}

#[test]
fn unit_test_scalar_multiplication(){
    
}