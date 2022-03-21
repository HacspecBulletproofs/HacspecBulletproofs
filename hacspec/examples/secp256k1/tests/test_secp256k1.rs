use hacspec_lib::*;
#[allow(unused_imports)]
use hacspec_secp256k1::*;
use hacspec_dev::prelude::*;

extern crate quickcheck;
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

include!("../src/secp256k1_generators.txt");

#[test]
fn test_infty_add_1() {
    fn helper(p: AffineGenerator) -> bool {
        let p = p.into();
        let res = add_points(p, INFINITY());
        res == p
    }
    QuickCheck::new()
        .tests(5)
        .quickcheck(helper as fn(AffineGenerator) -> bool);
}


#[test]
fn test_infty_add_2() {
    fn helper(p: AffineGenerator) -> bool {
        let p = p.into();
        let res = add_points(INFINITY(), p);
        res == p
    }
    QuickCheck::new()
        .tests(5)
        .quickcheck(helper as fn(AffineGenerator) -> bool);
}

#[test]
fn test_infty_add_3() {
    let res = add_points(INFINITY(), INFINITY());
    assert!(res == INFINITY())
}

#[test]
fn test_add_negatives_gives_infty() {
    fn helper(p: AffineGenerator) -> bool {
        let p = p.into();
        let minus_p = neg_point(p);
        let res = add_points(p, minus_p);
        res == INFINITY()
    }
    QuickCheck::new()
        .tests(5)
        .quickcheck(helper as fn(AffineGenerator) -> bool);
}

#[test]
fn base_point_on_curve() {
    assert!(is_point_on_curve(BASE_POINT()))
}

#[test]
fn two_base_point_on_curve() {
    assert!(is_point_on_curve(double_point(BASE_POINT())))
}

#[test]
fn n_base_point_on_curve() {
    fn helper(k: Secp256k1ScalarGenerator) -> bool {
        let k = k.into();
        is_point_on_curve(scalar_multiplication(k, BASE_POINT()))
    }
    QuickCheck::new()
        .tests(5)
        .quickcheck(helper as fn(Secp256k1ScalarGenerator) -> bool);
}

#[test]
fn test_associativity() {
    fn helper(p: AffineGenerator, q: AffineGenerator, r: AffineGenerator) -> bool {
        let p = p.into();
        let q = q.into();
        let r = r.into();
        add_points(add_points(p, q), r) == add_points(p, add_points(q, r))
    }
    QuickCheck::new()
        .tests(5)
        .quickcheck(helper as fn(AffineGenerator, AffineGenerator, AffineGenerator) -> bool);
}

#[test]
fn test_commutativity() {
    fn helper(p: AffineGenerator, q: AffineGenerator) -> bool {
        let p = p.into();
        let q = q.into();
        add_points(p, q) == add_points(q, p)
    }
    QuickCheck::new()
        .tests(5)
        .quickcheck(helper as fn(AffineGenerator, AffineGenerator) -> bool);
}

#[test]
fn test_distributive_scalar_multiplication() {
    fn helper(p: AffineGenerator, k1: Secp256k1ScalarGenerator, k2: Secp256k1ScalarGenerator) -> bool {
        let p = p.into();
        let k1 = k1.into();
        let k2 = k2.into();
        let k = k1 + k2;
        let k1p = scalar_multiplication(k1, p);
        let k2p = scalar_multiplication(k2, p);
        let kp = scalar_multiplication(k, p);
        add_points(k1p, k2p) == kp
    }
    QuickCheck::new()
        .tests(5)
        .quickcheck(helper as fn(AffineGenerator, Secp256k1ScalarGenerator, Secp256k1ScalarGenerator) -> bool);
}

#[test]
fn test_generated_points_on_curve() {
    fn helper(p: AffineGenerator) -> TestResult {
        let p = p.into();
        if is_infinity(p) {
            return TestResult::discard()
        }
        TestResult::from_bool(is_point_on_curve(p))
    }
    QuickCheck::new()
        .tests(5)
        .min_tests_passed(4)
        .max_tests(10)
        .quickcheck(helper as fn(AffineGenerator) -> TestResult);
}