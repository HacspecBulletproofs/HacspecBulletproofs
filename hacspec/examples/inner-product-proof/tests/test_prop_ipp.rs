#![allow(non_snake_case)]
use rand::{distributions::Uniform, Rng}; // 0.6.5

//use rand_core::{CryptoRng, RngCore};

use hacspec_ipp::*;
use hacspec_lib::*;
use hacspec_ristretto::*;

use curve25519_dalek_ng::ristretto::CompressedRistretto as DalekRistrettoPointEncoded;
use curve25519_dalek_ng::ristretto::RistrettoPoint as DalekRistrettoPoint;
use curve25519_dalek_ng::scalar::Scalar as DalekScalar;
use quickcheck::*;

use bulletproofs::inner_product_proof::InnerProductProof;

//use curve25519_dalek_ng::traits::VartimeMultiscalarMul;

// === Helper functions === //

fn quickcheck(n: u64, helper: impl Testable) {
    QuickCheck::new()
        .tests(n)
        .min_tests_passed(n)
        .max_tests(1000000)
        .quickcheck(helper);
}

fn vec_to_arr(v: Vec<u8>) -> [u8; 64] {
    let mut arr: [u8; 64] = [0; 64];

    for i in 0..v.len() {
        arr[i] = v[i];
    }
    arr
}

fn vec_to_seq(xs: &Vec<u8>) -> Seq<U8> {
    let mut ret = Seq::<U8>::new(xs.len());
    for i in 0..xs.len() {
        ret[i] = U8::classify(xs[i])
    }
    ret
}

fn seq_to_vec(xs: &Seq<U8>) -> Vec<u8> {
    let mut ret = Vec::<u8>::with_capacity(xs.len());
    for i in 0..xs.len() {
        ret.push(xs[i].declassify())
    }
    ret
}

fn exp(xs: &Vec<u8>, y: usize) -> Vec<u8> {
    let xs_prime = hac_from_bytes(&xs).pow(y as u128);
    seq_to_vec(&xs_prime.to_byte_seq_le())
}

fn hac_from_bytes(xs: &Vec<u8>) -> Scalar {
    Scalar::from_byte_seq_le(vec_to_seq(&xs))
}

fn dal_from_bytes(xs: &Vec<u8>) -> DalekScalar {
    DalekScalar::from_bytes_mod_order_wide(&vec_to_arr(xs.clone()))
}

fn cmp_bytes(hacs: Seq<U8>, dals: Vec<u8>) -> bool {
    let hacs = hacs.declassify();
    let mut b = true;

    for i in 0..hacs.len() {
        if hacs[i] != dals[i] {
            b = false
        }
    }
    b
}

fn cmp_encoded_points(hac: RistrettoPointEncoded, dal: DalekRistrettoPointEncoded) -> bool {
    cmp_bytes(hac.to_le_bytes(), dal.as_bytes().to_vec())
}

fn cmp_scalars(hac: Scalar, dal: DalekScalar) -> bool {
    let hacs = hac.to_byte_seq_le();
    let dals = dal.to_bytes();

    cmp_bytes(hacs, dals.to_vec())
}

// === Tests === //

/*
 * The following test tests private functions of the dalek-bulletproofs library
 * It is commented out since it requires forking the external library and changing
 * the associated methods and structs to be made public, but the test does pass.
 *
 * Note that the function is also tested in the hacspec-bulletproofs crate.
 */

#[test]
fn testquick() {
    fn helper() -> TestResult {
        // Init randomness
        let mut rng = rand::thread_rng();
        let byte_range = Uniform::new(0, 255);
        let n_range = Uniform::new(2, 6);

        // Generate n as a random bounded power of 2
        let n = (2 as usize).pow(rng.sample(&n_range));

        // Initialize input variables
        let mut G: Vec<Vec<u8>> = Vec::with_capacity(n);
        let mut H: Vec<Vec<u8>> = Vec::with_capacity(n);
        let mut H_factors: Vec<Vec<u8>> = Vec::with_capacity(n);
        let mut a: Vec<Vec<u8>> = Vec::with_capacity(n);
        let mut b: Vec<Vec<u8>> = Vec::with_capacity(n);

        // Fill Variables with randomness
        let Q: Vec<u8> = (0..64).map(|_| rng.sample(&byte_range)).collect();
        let y_inv: Vec<u8> = (0..32).map(|_| rng.sample(&byte_range)).collect();

        for i in 0..n {
            G.push((0..64).map(|_| rng.sample(&byte_range)).collect());
            H.push((0..64).map(|_| rng.sample(&byte_range)).collect());
            H_factors.push(exp(&y_inv, i));
            a.push((0..32).map(|_| rng.sample(&byte_range)).collect());
            b.push((0..32).map(|_| rng.sample(&byte_range)).collect());
        }

        // Initialize IPP inputs
        let mut G_hac: Seq<RistrettoPoint> = Seq::new(n);
        let mut H_hac: Seq<RistrettoPoint> = Seq::new(n);
        let mut Gf_hac: Seq<Scalar> = Seq::new(n);
        let mut Hf_hac: Seq<Scalar> = Seq::new(n);
        let mut a_hac: Seq<Scalar> = Seq::new(n);
        let mut b_hac: Seq<Scalar> = Seq::new(n);
        let mut b_prime_hac: Seq<Scalar> = Seq::new(n);
        let mut c_hac = Scalar::ZERO();
        let mut P1_hac = IDENTITY_POINT();
        let mut P2_hac = IDENTITY_POINT();

        let mut G_dal: Vec<DalekRistrettoPoint> = Vec::with_capacity(n);
        let mut H_dal: Vec<DalekRistrettoPoint> = Vec::with_capacity(n);
        let mut Gf_dal: Vec<DalekScalar> = Vec::with_capacity(n);
        let mut Hf_dal: Vec<DalekScalar> = Vec::with_capacity(n);
        let mut a_dal: Vec<DalekScalar> = Vec::with_capacity(n);
        let mut b_dal: Vec<DalekScalar> = Vec::with_capacity(n);
        let mut b_prime_dal: Vec<DalekScalar> = Vec::with_capacity(n);
        let mut c_dal = DalekScalar::zero();
        let mut P1_dal =
            DalekScalar::zero() * curve25519_dalek_ng::constants::RISTRETTO_BASEPOINT_POINT;
        let mut P2_dal =
            DalekScalar::zero() * curve25519_dalek_ng::constants::RISTRETTO_BASEPOINT_POINT;

        // Generate inputs from random bytevecs
        for i in 0..n {
            G_hac[i] = one_way_map(ByteString::from_public_slice(&G[i]));
            H_hac[i] = one_way_map(ByteString::from_public_slice(&H[i]));
            Gf_hac[i] = Scalar::ONE();
            Hf_hac[i] = hac_from_bytes(&H_factors[i]);
            a_hac[i] = hac_from_bytes(&a[i]);
            b_hac[i] = hac_from_bytes(&b[i]);
            b_prime_hac[i] = b_hac[i] * Hf_hac[i];
            c_hac = c_hac + a_hac[i] * b_hac[i];
            P1_hac = add(P1_hac, mul(a_hac[i], G_hac[i]));
            P2_hac = add(P2_hac, mul(b_prime_hac[i], H_hac[i]));

            G_dal.push(DalekRistrettoPoint::from_uniform_bytes(&vec_to_arr(
                G[i].clone(),
            )));
            H_dal.push(DalekRistrettoPoint::from_uniform_bytes(&vec_to_arr(
                H[i].clone(),
            )));
            Gf_dal.push(DalekScalar::one());
            Hf_dal.push(dal_from_bytes(&H_factors[i]));
            a_dal.push(dal_from_bytes(&a[i]));
            b_dal.push(dal_from_bytes(&b[i]));
            b_prime_dal.push(b_dal[i] * Hf_dal[i]);
            c_dal += a_dal[i] * b_dal[i];
            P1_dal = P1_dal + a_dal[i] * G_dal[i];
            P2_dal = P2_dal + b_prime_dal[i] * H_dal[i];
        }

        let Q_hac = one_way_map(ByteString::from_public_slice(&Q));
        let P3_hac = mul(c_hac, Q_hac);
        let P_hac = add(P1_hac, add(P2_hac, P3_hac));

        let Q_dal = DalekRistrettoPoint::from_uniform_bytes(&vec_to_arr(Q));
        let P3_dal = c_dal * Q_dal;
        let P_dal = P1_dal + P2_dal + P3_dal;

        let transcript_hac = hacspec_merlin::new(Seq::<U8>::new(0));
        let mut transcript_dal = merlin::Transcript::new(b"");

        // Run IPP algorithms
        let transcript_ipp_hac = create(
            transcript_hac,
            Q_hac.clone(),
            Gf_hac.clone(),
            Hf_hac.clone(),
            G_hac.clone(),
            H_hac.clone(),
            a_hac,
            b_hac,
        )
        .unwrap();
        let (transcript_hac, ipp_hac) = transcript_ipp_hac;
        let (a_hac, b_hac, L_vec_hac, R_vec_hac) = ipp_hac.clone();

        let ipp_dal = InnerProductProof::create(
            &mut transcript_dal,
            &Q_dal,
            &Gf_dal,
            &Hf_dal,
            G_dal.clone(),
            H_dal.clone(),
            a_dal,
            b_dal,
        );

        assert!(cmp_scalars(a_hac, ipp_dal.a));
        assert!(cmp_scalars(b_hac, ipp_dal.b));

        assert_eq!(L_vec_hac.len(), ipp_dal.L_vec.len());
        assert_eq!(R_vec_hac.len(), ipp_dal.R_vec.len());
        for i in 0..L_vec_hac.len() {
            assert!(cmp_encoded_points(L_vec_hac[i], ipp_dal.L_vec[i]));
            assert!(cmp_encoded_points(R_vec_hac[i], ipp_dal.R_vec[i]));
        }

        let verScalars_hac =
            verification_scalars(ipp_hac.clone(), n, transcript_hac.clone()).unwrap();
        let verScalars_dal = ipp_dal
            .verification_scalars(n, &mut transcript_dal)
            .unwrap();
        println!(
            "len: {}, {}",
            verScalars_hac.0.len(),
            verScalars_dal.0.len()
        );
        println!(
            "len: {}, {}",
            verScalars_hac.1.len(),
            verScalars_dal.1.len()
        );
        println!(
            "len: {}, {}",
            verScalars_hac.2.len(),
            verScalars_dal.2.len()
        );

        assert_eq!(verScalars_hac.0.len(), verScalars_dal.0.len());
        assert_eq!(verScalars_hac.1.len(), verScalars_dal.1.len());
        assert_eq!(verScalars_hac.2.len(), verScalars_dal.2.len());

        for i in 0..verScalars_hac.0.len() {
            assert!(cmp_scalars(verScalars_hac.0[i], verScalars_dal.0[i]));
            assert!(cmp_scalars(verScalars_hac.1[i], verScalars_dal.1[i]));
        }
        for i in 0..verScalars_hac.2.len() {
            assert!(cmp_scalars(verScalars_hac.2[i], verScalars_dal.2[i]));
        }

        // Verify proofs

        let mut verifier = merlin::Transcript::new(b"");
        let verifier_hac = hacspec_merlin::new(Seq::<U8>::new(0));

        let ver_hac = verify(
            ipp_hac,
            n,
            verifier_hac,
            Gf_hac,
            Hf_hac,
            P_hac,
            Q_hac,
            G_hac,
            H_hac,
        );

        let ver_dal = ipp_dal.verify(
            n,
            &mut verifier,
            Gf_dal,
            Hf_dal,
            &P_dal,
            &Q_dal,
            &G_dal,
            &H_dal,
        );

        assert!(ver_dal.is_ok());
        assert!(ver_hac.is_ok());

        TestResult::from_bool(true)
    }
    quickcheck(1, helper as fn() -> TestResult)
}

/*
#[test]
fn test_helper_create() {
    let n = 4;
    let mut rng = rand::thread_rng();

    //use bulletproofs::generators::BulletproofGens;
    //use bulletproofs::*;
    //let bp_gens = BulletproofGens::new(n, 1);
    let byte_range = Uniform::new(0, 255);

    //let G: Vec<DalekRistrettoPoint> = bp_gens.share(0).G(n).cloned().collect();
    //let H: Vec<DalekRistrettoPoint> = bp_gens.share(0).H(n).cloned().collect();

    // Initialize IPP inputs
    let mut G: Vec<DalekRistrettoPoint> = Vec::with_capacity(n);
    let mut H: Vec<DalekRistrettoPoint> = Vec::with_capacity(n);
    let mut a: Vec<DalekScalar> = Vec::with_capacity(n);
    let mut b: Vec<DalekScalar> = Vec::with_capacity(n);
    let mut G_factors: Vec<DalekScalar> = Vec::with_capacity(n);
    let mut H_factors: Vec<DalekScalar> = Vec::with_capacity(n);
    let mut b_prime: Vec<DalekScalar> = Vec::with_capacity(n);
    let mut a_prime: Vec<DalekScalar> = Vec::with_capacity(n);
    let mut c = DalekScalar::zero();
    let mut P1 = DalekScalar::zero() * curve25519_dalek_ng::constants::RISTRETTO_BASEPOINT_POINT;
    let mut P2 = DalekScalar::zero() * curve25519_dalek_ng::constants::RISTRETTO_BASEPOINT_POINT;

    let y_inv: Vec<u8> = (0..32).map(|_| rng.sample(&byte_range)).collect();
    // Generate inputs from random bytevecs
    for i in 0..n {
        G.push(DalekRistrettoPoint::from_uniform_bytes(&vec_to_arr(
            (0..64).map(|_| rng.sample(&byte_range)).collect(),
        )));
        H.push(DalekRistrettoPoint::from_uniform_bytes(&vec_to_arr(
            (0..64).map(|_| rng.sample(&byte_range)).collect(),
        )));
        a.push(dal_from_bytes(
            &(0..32).map(|_| rng.sample(&byte_range)).collect(),
        ));
        b.push(dal_from_bytes(
            &(0..32).map(|_| rng.sample(&byte_range)).collect(),
        ));
        G_factors.push(DalekScalar::one());
        H_factors.push(dal_from_bytes(&exp(&y_inv, i)));
        b_prime.push(b[i] * H_factors[i]);
        c += a[i] * b[i];
        P1 = P1 + a[i] * G[i];
        P2 = P2 + b_prime[i] * H[i];
    }

    let Q = DalekRistrettoPoint::from_uniform_bytes(&vec_to_arr(
        (0..64).map(|_| rng.sample(&byte_range)).collect(),
    ));
    let y_inv = dal_from_bytes(&y_inv);
    //let c = inner_product_dalek(&a, &b);
    let P3 = c * Q;
    let P = P1 + P2 + P3;

    // Q would be determined upstream in the protocol, so we pick a random one.
    // a and b are the vectors for which we want to prove c = <a,b>

    //let G_factors: Vec<DalekScalar> = iter::repeat(DalekScalar::one()).take(n).collect();

    // y_inv is (the inverse of) a random challenge
    //let H_factors: Vec<DalekScalar> = util::exp_iter(y_inv).take(n).collect();

    // P would be determined upstream, but we need a correct P to check the proof.
    //
    // To generate P = <a,G> + <b,H'> + <a,b> Q, compute
    //             P = <a,G> + <b',H> + <a,b> Q,
    // where b' = b \circ y^(-n)
    //let b_prime = b.iter().zip(util::exp_iter(y_inv)).map(|(bi, yi)| bi * yi);
    // a.iter() has Item=&Scalar, need Item=Scalar to chain with b_prime
    let a_prime = a.iter().cloned();

    //let P = DalekRistrettoPoint::vartime_multiscalar_mul(
    //    a_prime.chain(b_prime).chain(iter::once(c)),
    //    G.iter().chain(H.iter()).chain(iter::once(&Q)),
    //);

    let mut verifier = merlin::Transcript::new(b"innerproducttest");
    let proof = InnerProductProof::create(
        &mut verifier,
        &Q,
        &G_factors,
        &H_factors,
        G.clone(),
        H.clone(),
        a.clone(),
        b.clone(),
    );

    let mut verifier = merlin::Transcript::new(b"innerproducttest");
    assert!(proof
        .verify(
            n,
            &mut verifier,
            iter::repeat(DalekScalar::one()).take(n),
            bulletproofs::util::exp_iter(y_inv).take(n),
            &P,
            &Q,
            &G,
            &H
        )
        .is_ok());

    let proof = InnerProductProof::from_bytes(proof.to_bytes().as_slice()).unwrap();
    let mut verifier = merlin::Transcript::new(b"innerproducttest");
    assert!(proof
        .verify(
            n,
            &mut verifier,
            iter::repeat(DalekScalar::one()).take(n),
            bulletproofs::util::exp_iter(y_inv).take(n),
            &P,
            &Q,
            &G,
            &H
        )
        .is_ok());
}
*/
