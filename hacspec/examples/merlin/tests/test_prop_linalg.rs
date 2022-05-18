extern crate quickcheck;

use hacspec_lib::*;
use hacspec_merlin::*;

use merlin::*;
use quickcheck::*;

// === Helper functions ===

fn quickcheck(helper: impl Testable) {
	QuickCheck::new()
		.tests(100)
		.min_tests_passed(100)
		.max_tests(1000000)
		.quickcheck(helper);
} 

fn b2(s: &str) -> &[u8] {
	assert!(s.is_ascii());
	s.as_bytes()
}

fn b1(s: &str) -> Seq<U8> {
	let xs = Seq::<u8>::from_vec(s.to_string().into_bytes());

	let mut ret = Seq::<U8>::new(xs.len());
	for i in 0..xs.len() {
		ret[i] = U8::classify(xs[i])
	}
	ret
}

fn f(xs: Vec<u8>) -> Seq<U8> {
	let mut ret = Seq::<U8>::new(xs.len());
	for i in 0..xs.len() {
		ret[i] = U8::classify(xs[i])
	}
	ret
}


/*
#[test]
fn test() {
	let label = "Conformance Test Protocol";
	let mut s1 = new(b1(label));
	let mut s2 = merlin::Strobe128::new(b2(label));

	let msg = [99u8; 1024];
	s1 = meta_ad(s1, b1("ms"), false);
	s1 = meta_ad(s1, b1("g"), true);
	s1 = ad(s1, f(Vec::<u8>::from(msg)), false);

	s2.meta_ad(b"ms", false);
	s2.meta_ad(b"g", true);
	s2.ad(&msg, false);

	for i in 0..s1.0.len() {
		assert_eq!(s1.0[i].declassify(), s2.state[i])
	}

	s1 = meta_ad(s1, b1("prf"), false);
	let (s1, prf1) = prf(s1, Seq::<U8>::new(32), false);

	let mut prf2 = [0u8; 32];
	s2.meta_ad(b"prf", false);
	s2.prf(&mut prf2, false);

	for i in 0..prf1.len() {
		assert_eq!(prf1[i].declassify(), prf2[i])
	}
}

*/
