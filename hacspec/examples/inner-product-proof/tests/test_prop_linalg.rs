extern crate quickcheck;
extern crate bulletproofs;

use hacspec_inner_product_proof::*;
use hacspec_lib::*;
use hacspec_ristretto as ristretto;
use hacspec_ristretto::*;

use quickcheck::*;
use bulletproofs::*;

// === Helper functions ===

fn quickcheck(helper: impl Testable) {
    QuickCheck::new()
        .tests(100)
        .min_tests_passed(100)
        .max_tests(1000000)
        .quickcheck(helper);
}

#[test]
fn test() {
    let Q = BASE_POINT();
    let G = Seq::new(10);
    let H = Seq::new(10);
    let G_factors = Seq::new(10);
    let H_factors = Seq::new(10);
    let a = Seq::new(10);
    let b = Seq::new(10);

    create("".into(), Q, G_factors, H_factors, G, H, a, b);
    assert!(true)
}

#[test]
fn test2() {
    assert!(true)
}
