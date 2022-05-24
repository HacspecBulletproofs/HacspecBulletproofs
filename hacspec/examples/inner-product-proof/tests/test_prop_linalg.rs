extern crate quickcheck;
extern crate bulletproofs;

use hacspec_inner_product_proof::*;
use hacspec_lib::*;
use hacspec_ristretto as ristretto;
use hacspec_ristretto::*;

use hacspec_merlin::*;

use quickcheck::*;
use bulletproofs::*;
use merlin::*;
use curve25519_dalek_ng::constants::*;
use curve25519_dalek_ng::ristretto as dalek_ristretto;

use bulletproofs::inner_product_proof::InnerProductProof;

// === Helper functions ===

fn quickcheck(helper: impl Testable) {
    QuickCheck::new()
        .tests(10)
        .min_tests_passed(100)
        .max_tests(1000000)
        .quickcheck(helper);
}

fn cmp_points(p: RistrettoPoint, q: dalek_ristretto::CompressedRistretto) -> bool {
	let p_enc = encode(p);
	let p_bytes = p_enc.to_le_bytes();
	let p_native = p_bytes.to_native();
	let p_slice = p_native.as_slice();

	let q_slice = q.to_bytes();

	q_slice == p_slice
}

// Rename this
fn f(xs: Vec<u8>) -> Seq<U8> {
	let mut ret = Seq::<U8>::new(xs.len());
	for i in 0..xs.len() {
		ret[i] = U8::classify(xs[i])
	}
	ret
}

public_nat_mod!(
    type_name: LocalFieldElement,
    type_of_canvas: FieldCanvas,
    bit_size_of_field: 512,
    modulo_value: "1000000000000000000000000000000014def9dea2f79cd65812631a5cf5d3ed"
);

//#[test]
fn test() {
	let transcript = hacspec_merlin::new(Seq::<U8>::new(0));
	let n = 16;
	let Q = BASE_POINT();
	let mut G = Seq::new(n);
	let mut H = Seq::new(n);
	let mut G_factors = Seq::new(n);
	let mut H_factors = Seq::new(n);
	let mut a = Seq::new(n);
	let mut b = Seq::new(n);
	for i in 0..G_factors.len() {
		G[i] = BASE_POINT();
		H[i] = BASE_POINT();
		G_factors[i] = FieldElement::ZERO();
		H_factors[i] = FieldElement::ZERO();
		a[i] = FieldElement::ZERO();
		b[i] = FieldElement::ZERO();
	}

	let (a_hac, b_hac, G_hac, H_hac) = create(transcript, Q, G_factors, H_factors, G, H, a, b).unwrap();
	let mut transcript = merlin::Transcript::new(b"");
	let Q = RISTRETTO_BASEPOINT_POINT;
	let G_factors = vec![curve25519_dalek_ng::scalar::Scalar::zero(); n];
	let H_factors = vec![curve25519_dalek_ng::scalar::Scalar::zero(); n];
	let G = vec![RISTRETTO_BASEPOINT_POINT; n];
	let H = vec![RISTRETTO_BASEPOINT_POINT; n];
	let a = vec![curve25519_dalek_ng::scalar::Scalar::zero(); n];
	let b = vec![curve25519_dalek_ng::scalar::Scalar::zero(); n];
	let dal = bulletproofs::inner_product_proof::InnerProductProof::create(&mut transcript, &Q, &G_factors, &H_factors, G, H, a, b);
	let a_dal = dal.a;

	//println!("{}", res.unwrap());
	//let hac = a_hac[i].to_byte_seq_le();
	//let dal = dal.a[i].to_bytes_le();
	let a_hacs = a_hac.to_byte_seq_le();
	let b_hacs = b_hac.to_byte_seq_le();
	let a_dals = dal.a.to_bytes();
	let b_dals = dal.b.to_bytes();

	for i in 0..32 {
		assert_eq!((a_hacs[i] as U8).declassify(), a_dals[i]);
		assert_eq!((b_hacs[i] as U8).declassify(), b_dals[i]);
	}

	for i in 0..G_hac.len() {
		assert!(cmp_points(G_hac[i], dal.L_vec[i]));
		assert!(cmp_points(H_hac[i], dal.R_vec[i]));
	}

	//for i in 0..hac.len() {
	//	assert_eq!(hac[i], dal[i])
	//}
}

#[test]
fn test2() {
	//let t1 = hacspec_merlin::new(Seq::<U8>::new(0));
	//let mut t2 = merlin::Transcript::new(b"");

	////t1 = inner_product_domain_sep(t1, 123)
	////t2.inner_product_domain_sep(123)
	//let mut arr: [u8; 64] = [0; 64];
	//arr[32] = 12;
	//arr.to_vec();
	//let x = curve25519_dalek_ng::scalar::Scalar::from_bytes_mod_order_wide(&arr);
	////let y = LocalScalar::from_byte_seq_le(f(arr.to_vec()));

	//let x_ = curve25519_dalek_ng::scalar::Scalar::zero() - curve25519_dalek_ng::scalar::Scalar::one();
	//let y_ = FieldElement::ZERO() - FieldElement::ONE();
	//println!("{:?}", x);
	////println!("{:?}", x_);
	////println!("{:?}", y.to_byte_seq_le().iter());
	////println!("{:?}", y_.to_byte_seq_le().iter());

	//assert!(false)
}
