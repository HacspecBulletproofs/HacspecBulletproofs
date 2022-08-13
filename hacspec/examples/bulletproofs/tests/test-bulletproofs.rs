// The #-commented lines are hidden in Rustdoc but not in raw
// markdown rendering, and contain boilerplate code so that the
// code in the README.md is actually run as part of the test suite.
#![allow(non_snake_case)]
use hacspec_lib::*;
use hacspec_ristretto::*;
use hacspec_ristretto::Scalar as hac_scalar;
use hacspec_ipp::InnerProductProof as hac_ipp;

extern crate rand;
use rand::*;

extern crate curve25519_dalek_ng;
use curve25519_dalek_ng::scalar::Scalar as rust_scalar;
use curve25519_dalek_ng::ristretto::CompressedRistretto;

extern crate bulletproofs;
use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};

// HELPER FUNCTIONS //

fn convert(point: curve25519_dalek_ng::ristretto::RistrettoPoint) -> RistrettoPoint {
    decode(RistrettoPointEncoded::from_public_array(point.compress().to_bytes())).unwrap()

}

fn create_bp_gens(number_of_values: usize, n:usize) -> ((usize, usize, Seq<Seq<RistrettoPoint>>, Seq<Seq<RistrettoPoint>>), BulletproofGens) {
    let bp_gens_rust = BulletproofGens::new(n,number_of_values);

    let mut bp_gens_G_vec_hac = Seq::<Seq<RistrettoPoint>>::new(number_of_values);
    let mut bp_gens_H_vec_hac = Seq::<Seq<RistrettoPoint>>::new(number_of_values);
    for i in 0..number_of_values {
        bp_gens_G_vec_hac[i] = Seq::<RistrettoPoint>::new(n);
        bp_gens_H_vec_hac[i] = Seq::<RistrettoPoint>::new(n);
        for j in 0..n{
            bp_gens_G_vec_hac[i][j] = convert(bp_gens_rust.G_vec[i][j]);
            bp_gens_H_vec_hac[i][j] = convert(bp_gens_rust.H_vec[i][j]);
        }
    }
    let bp_gens_hac = (number_of_values, n, bp_gens_G_vec_hac, bp_gens_H_vec_hac);
    (bp_gens_hac,bp_gens_rust)
}

fn create_pc_gens() -> ((RistrettoPoint,RistrettoPoint), PedersenGens) {
    let pc_gens_rust = PedersenGens::default();
    let pc_gens_blinding_hac =  decode(RistrettoPointEncoded::from_hex("8c9240b456a9e6dc65c377a1048d745f94a08cdb7f44cbcd7b46f34048871134")).unwrap(); //This is the hex for the point used by PedersenGens::default()
    let pc_gens_base_hac = BASE_POINT();
    ((pc_gens_base_hac,pc_gens_blinding_hac), pc_gens_rust)
}


fn create_transcript() -> (hacspec_merlin::Transcript, merlin::Transcript) {
    let transcript_rust = merlin::Transcript::new(b"test");
    let transcript_hac = hacspec_merlin::new(byte_seq!(116u8, 101u8, 115u8, 116u8));
    (transcript_hac, transcript_rust)

}

fn generate_random_values(n:usize, max: usize) -> (Seq<u64>, Vec<u64>) {

    let mut hac_seq = Seq::<u64>::new(n);
    let mut rust_vec = Vec::<u64>::new();
    let mut rng = rand::thread_rng();
    if max == 8usize {
        for i in 0..n {
            let random: u8 = rng.gen();
            hac_seq[i] = random as u64;
            rust_vec.push(random as u64);
        }
    }
    else if max == 16usize {
        for i in 0..n {
            let random: u16 = rng.gen();
            hac_seq[i] = random as u64;
            rust_vec.push(random as u64);
        }
    }
    else if max == 32usize{
        for i in 0..n {
            let random: u32 = rng.gen();
            hac_seq[i] = random as u64;
            rust_vec.push(random as u64);
        }
    }
    else {
        for i in 0..n {
            let random: u64 = rng.gen();
            hac_seq[i] = random;
            rust_vec.push(random);
        }
    }
    (hac_seq,rust_vec)
}

fn random_scalar() -> (hac_scalar, rust_scalar) {
    let mut rng = rand::thread_rng();
    let random: u64 = rng.gen();
    (hac_scalar::from_literal(random as u128), rust_scalar::from(random))
}

fn generate_random_scalars(n: usize) -> (Seq<hac_scalar>, Vec<rust_scalar>) {

    let mut hac_seq = Seq::<hac_scalar>::new(n);
    let mut rust_vec = Vec::<rust_scalar>::new();

    for i in 0..n {
        let (rand_hac, rand_rust) = random_scalar();
        hac_seq[i] = rand_hac;
        rust_vec.push(rand_rust);
    }

    (hac_seq,rust_vec)
}

fn generate_many_random_scalars(n:usize, number_of_arrays: usize) -> (Seq<Seq<hac_scalar>>,Vec<Vec<rust_scalar>>) {
    let mut rust_values = Vec::<Vec<rust_scalar>>::new();
    let mut hac_values = Seq::<Seq<hac_scalar>>::new(number_of_arrays);

    for i in 0..number_of_arrays {
        let (hac_array, rust_array) = generate_random_scalars(n);
        rust_values.push(rust_array);
        hac_values[i] = hac_array;
    }
    (hac_values,rust_values)

}

fn compare_encoded_points(hac: hacspec_ristretto::RistrettoPointEncoded, rust: curve25519_dalek_ng::ristretto::CompressedRistretto) -> bool {

    let hac_bytes = hac.to_le_bytes();
    let hac_native = hac_bytes.to_native();
    let hac_slice = hac_native.as_slice();
    
    let rust_slice = rust.to_bytes();
    
    hac_slice == rust_slice
}

fn compare_scalars(hac: hac_scalar, rust: rust_scalar) -> bool {
    hac.to_le_bytes() == rust.as_bytes()
}

fn compare_ipp(hac: hac_ipp, rust: bulletproofs::inner_product_proof::InnerProductProof) -> bool {
    let (a,b,L_vec,R_vec) = hac;

    compare_scalars(a,rust.a) &
    compare_scalars(b,rust.b) &
    compare_seqs(L_vec, rust.L_vec) &
    compare_seqs(R_vec, rust.R_vec)
}


fn compare_proofs(hac: hacspec_bulletproofs::RangeProof, rust: bulletproofs::RangeProof) -> bool {
    
    let (A_hac, S_hac, T_1_hac, T_2_hac, t_x_hac, t_x_blinding_hac, e_blinding_hac, ipp_hac) = hac;

    let res = compare_encoded_points(A_hac, rust.A)
        & compare_encoded_points(S_hac, rust.S)
        & compare_encoded_points(T_1_hac, rust.T_1)
        & compare_encoded_points(T_2_hac, rust.T_2)
        & compare_scalars(t_x_hac, rust.t_x)
        & compare_scalars(t_x_blinding_hac, rust.t_x_blinding)
        & compare_scalars(e_blinding_hac, rust.e_blinding)
        & compare_ipp(ipp_hac, rust.ipp_proof);
        

    res
}

fn compare_seqs(hac: Seq<RistrettoPointEncoded>, rust: Vec<CompressedRistretto>) -> bool {
    if hac.len() != rust.len() {
        return false
    }
    let mut res = true;
    for i in 0..hac.len() {
        res = res && compare_encoded_points(hac[i], rust[i]);
    }
    return res
}

fn test_bulletproofs(number_of_values: usize, n: usize) {

    let (bp_gens_hac, bp_gens_rust) = create_bp_gens(number_of_values, n);
    let (pc_gens_hac, pc_gens_rust) = create_pc_gens();
    let (transcript_hac, mut transcript_rust) = create_transcript();
    
    let (values_hac, values_rust) = generate_random_values(number_of_values, n);
    let (blindings_hac, blindings_rust) = generate_random_scalars(number_of_values);
    let (a_blindings_hac, a_blindings_rust) = generate_random_scalars(number_of_values);
    let (s_blindings_hac, s_blindings_rust) = generate_random_scalars(number_of_values);
    let (s_L_hac, s_L_rust) = generate_many_random_scalars(n, number_of_values);
    let (s_R_hac, s_R_rust) = generate_many_random_scalars(n, number_of_values);
    let (t1_blindings_hac, t1_blindings_rust) = generate_random_scalars(number_of_values);
    let (t2_blindings_hac, t2_blindings_rust) = generate_random_scalars(number_of_values);


    let (proof_rust, committed_values_rust) = RangeProof::prove_multiple_with_rng(
        &bp_gens_rust,
        &pc_gens_rust,
        &mut transcript_rust,
        &values_rust,
        &blindings_rust,
        n,
        a_blindings_rust,
        s_blindings_rust,
        s_L_rust,
        s_R_rust,
        t1_blindings_rust,
        t2_blindings_rust,
    ).expect("A real program could handle errors");

    let (proof_hac, committed_values_hac) = hacspec_bulletproofs::prove(
        bp_gens_hac.clone(),
        pc_gens_hac,
        transcript_hac,
        values_hac,
        blindings_hac,
        n,
        a_blindings_hac,
        s_blindings_hac,
        s_L_hac,
        s_R_hac,
        t1_blindings_hac,
        t2_blindings_hac,
    ).unwrap();

    assert!(compare_proofs(proof_hac.clone(), proof_rust.clone()) && compare_seqs(committed_values_hac.clone(),committed_values_rust.clone()));

    let (c_hac, c_rust) = random_scalar();
    let (verify_transcript_hac, mut verify_transcript_rust) = create_transcript();

    let verified_hac = hacspec_bulletproofs::verify(
        proof_hac, 
        bp_gens_hac,
        pc_gens_hac,
        verify_transcript_hac,
        committed_values_hac,
        n,
        c_hac
    );

    let verified_rust = proof_rust.verify_multiple_with_rng(
        &bp_gens_rust,
        &pc_gens_rust,
        &mut verify_transcript_rust,
        &committed_values_rust,
        n,
        c_rust
    );

    assert!(verified_rust.is_ok());
    assert!(verified_hac.is_ok());

}

// TEST FUNCTIONS:

#[test]
fn i2j8() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        test_bulletproofs(2, 8);
    }

    let elapsed = now.elapsed();
    println!("i2j8() completed succesfully");
    println!("Elapsed: {:.2?}", elapsed);
    assert!(false)
}

#[test]
fn i2j16() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        test_bulletproofs(2, 16);
    }

    let elapsed = now.elapsed();
    println!("i2j16() completed succesfully");
    println!("Elapsed: {:.2?}", elapsed);
    assert!(false)
}

#[test]
fn i2j32() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        test_bulletproofs(2, 32);
    }

    let elapsed = now.elapsed();
    println!("i2j32() completed succesfully");
    println!("Elapsed: {:.2?}", elapsed);
    assert!(false)
}

#[test]
fn i2j64() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        test_bulletproofs(2, 64);
    }

    let elapsed = now.elapsed();
    println!("i2j64() completed succesfully");
    println!("Elapsed: {:.2?}", elapsed);
    assert!(false)
}

#[test]
fn i4j8() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        test_bulletproofs(4, 8);
    }

    let elapsed = now.elapsed();
    println!("i4j8() completed succesfully");
    println!("Elapsed: {:.2?}", elapsed);
    assert!(false)
}

#[test]
fn i4j16() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        test_bulletproofs(4, 16);
    }

    let elapsed = now.elapsed();
    println!("i4j16() completed succesfully");
    println!("Elapsed: {:.2?}", elapsed);
    assert!(false)
}

#[test]
fn i4j32() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        test_bulletproofs(4, 32);
    }

    let elapsed = now.elapsed();
    println!("i4j32() completed succesfully");
    println!("Elapsed: {:.2?}", elapsed);
    assert!(false)
}

#[test]
fn i4j64() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        test_bulletproofs(4, 64);
    }

    let elapsed = now.elapsed();
    println!("i4j64() completed succesfully");
    println!("Elapsed: {:.2?}", elapsed);
    assert!(false)
}

#[test]
fn i8j8() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        test_bulletproofs(8, 8);
    }

    let elapsed = now.elapsed();
    println!("i8j8() completed succesfully");
    println!("Elapsed: {:.2?}", elapsed);
    assert!(false)
}

#[test]
fn i8j16() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        test_bulletproofs(8, 16);
    }

    let elapsed = now.elapsed();
    println!("i8j16() completed succesfully");
    println!("Elapsed: {:.2?}", elapsed);
    assert!(false)
}

#[test]
fn i8j32() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        test_bulletproofs(8, 32);
    }

    let elapsed = now.elapsed();
    println!("i8j32() completed succesfully");
    println!("Elapsed: {:.2?}", elapsed);
    assert!(false)
}

#[test]
fn i8j64() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        test_bulletproofs(8, 64);
    }

    let elapsed = now.elapsed();
    println!("i8j64() completed succesfully");
    println!("Elapsed: {:.2?}", elapsed);
    assert!(false)
}

#[test]
fn i16j8() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        test_bulletproofs(16, 8);
    }

    let elapsed = now.elapsed();
    println!("i16j8() completed succesfully");
    println!("Elapsed: {:.2?}", elapsed);
    assert!(false)
}

#[test]
fn i16j16() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        test_bulletproofs(16, 16);
    }

    let elapsed = now.elapsed();
    println!("i16j16() completed succesfully");
    println!("Elapsed: {:.2?}", elapsed);
    assert!(false)
}

#[test]
fn i16j32() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        test_bulletproofs(16, 32);
    }

    let elapsed = now.elapsed();
    println!("i16j32() completed succesfully");
    println!("Elapsed: {:.2?}", elapsed);
    assert!(false)
}

#[test]
fn i16j64() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        test_bulletproofs(16, 64);
    }

    let elapsed = now.elapsed();
    println!("i16j64() completed succesfully");
    println!("Elapsed: {:.2?}", elapsed);
    assert!(false)
}

#[test]
fn i32j64() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        test_bulletproofs(32, 64);
    }

    let elapsed = now.elapsed();
    println!("i32j64() completed succesfully");
    println!("Elapsed: {:.2?}", elapsed);
    assert!(false)
}

#[test]
fn i64j64() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        test_bulletproofs(64, 64);
    }

    let elapsed = now.elapsed();
    println!("i64j64() completed succesfully");
    println!("Elapsed: {:.2?}", elapsed);
    assert!(false)
}