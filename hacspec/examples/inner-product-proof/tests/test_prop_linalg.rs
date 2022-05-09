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
    let n = 16;
    let Q = BASE_POINT();
    let G = Seq::new(n);
    let H = Seq::new(n);
    let G_factors = Seq::new(n);
    let H_factors = Seq::new(n);
    let a = Seq::new(n);
    let b = Seq::new(n);

    let res = create("".into(), Q, G_factors, H_factors, G, H, a, b);
    assert!(res.is_ok())
}

#[test]
fn test2() {
    assert!(true)
}
