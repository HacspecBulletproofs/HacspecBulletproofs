#![allow(non_snake_case)]
extern crate quickcheck;
use hacspec_lib::*;
use curve25519_dalek::ristretto::*;
use hacspec_ristretto::*;
use quickcheck::*;

fn quickcheck(helper: impl Testable) {
    QuickCheck::new()
        .tests(10)
        .min_tests_passed(10)
        .max_tests(1000000)
        .quickcheck(helper);
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
fn unit_test_pos2_SQRT_RATIO() {
    let u = FieldElement::from_literal(715313025); //45*63*45*63
    let v = FieldElement::from_literal(89);
    let (was_SQRT,ratio) = SQRT_RATIO_M1(u,v);
    let expected_res = FieldElement::from_literal(2835);
    println!("{}",expected_res+ratio);
    assert_eq!(ratio,expected_res)
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
fn unit_test_neg_decode() {
    let x = FieldElement::from_literal(8);
    let y = FieldElement::from_literal(5);
    let z = FieldElement::from_literal(15);
    let t = FieldElement::from_literal(1);

    let m = (x,y,z,t);

    let encoded_m = encode(m);
    assert!(decode(encoded_m).is_err())
}

#[test]
fn unit_test_encode_decode() {
    let x = FieldElement::from_literal(1234);
    let y = FieldElement::from_literal(63627);
    let z = FieldElement::from_literal(1);
    let t = FieldElement::from_literal(1234125123521352);

    let m = (x,y,z,t);

    let decoded_encoded_m = decode(encode(m));

}

#[test]
fn unit_test_big_neg_decode() {
    let hexs = Seq::<&str>::from_vec(vec![
        //Non-canonical field encodings.
        //"00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        //"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
        //"f3ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
        //"edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",

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
        //"4eac077a713c57b4f4397629a4145982c661f48044dd3f96427d40b147d9742f",
        "de6a7b00deadc788eb6b6c8d20c0ae96c2f2019078fa604fee5b87d6e989ad7b",
        //"bcab477be20861e01e4a0e295284146a510150d9817763caf1a6f4b422d67042",
        //"2a292df7e32cababbd9de088d1d1abec9fc0440f637ed2fba145094dc14bea08",
        //"f4a9e534fc0d216c44b218fa0c42d99635a0127ee2e53c712f70609649fdff22",
        //"8268436f8c4126196cf64b3c7ddbda90746a378625f9813dd9b8457077256731",
        "2810e5cbc2cc4d4eece54f61c6f69758e289aa7ab440b3cbeaa21995c2f4232b",

        //Negative xy value.
        "3eb858e78f5a7254d8c9731174a94f76755fd3941c0ac93735c07ba14579630e",
        "a45fdc55c76448c049a1ab33f17023edfb2be3581e9c7aade8a6125215e04220",
        //"d483fe813c6ba647ebbfd3ec41adca1c6130c2beeee9d9bf065c8d151c5f396e",
        //"8a2e1d30050198c65a54483123960ccc38aef6848e1ec8f5f780e8523769ba32",
        //"32888462f8b486c68ad7dd9610be5192bbeaf3b443951ac1a8118419d9fa097b",
        "227142501b9d4355ccba290404bde41575b037693cef1f438c47f8fbf35d1165",
        "5c37cc491da847cfeb9281d407efc41e15144c876e0170b499a96a22ed31e01e",
        "445425117cb8c90edcbc7c1cc0e74f747f2c1efa5630a967c64f287792a48a4b",

        //s = -1, which causes y = 0.
        "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f"
    ]);

    hexs.iter().for_each(|x| {println!("{}",x); assert!(decode(EncodedPoint::from_hex(x)).is_err())});
}

#[test]
fn unit_test_point_addition() {

}

#[test]
fn unit_test_point_negation() {
    
}

#[test]
fn unit_test_scalar_multiplication(){
    
}