#![feature(int_log)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

use hacspec_lib::*;
use hacspec_merlin::*;
use hacspec_ristretto::*;

mod transcript;
use transcript::*;

mod errors;
use errors::*;

//a, b, L_vec, R_vec
pub type InnerProductProof = (
    Scalar,                     // a
    Scalar,                     // b
    Seq<RistrettoPointEncoded>, // L_vec
    Seq<RistrettoPointEncoded>, // R_vec
);

// Result types for brevity
type IppRes = Result<(Transcript, InnerProductProof), u8>;
type VerScalarsRes = Result<(Seq<Scalar>, Seq<Scalar>, Seq<Scalar>), u8>;
type VerifyRes = Result<(), u8>;
type DecodeRes = Result<RistrettoPoint, u8>;

// ASCI representations of: L, R, u
fn L_U8() -> Seq<U8> {
    byte_seq!(76u8)
}
fn R_U8() -> Seq<U8> {
    byte_seq!(82u8)
}
fn u_U8() -> Seq<U8> {
    byte_seq!(117u8)
}

// === Helper Functions === //

// The regular inner product of vectors u and v
fn inner_product(u: Seq<Scalar>, v: Seq<Scalar>) -> Scalar {
    let mut ret = Scalar::ZERO();

    for i in 0..u.len() {
        ret = ret + u[i] * v[i]
    }
    ret
}

// The scalar-sum of vectors xs and Xs
fn point_dot(xs: Seq<Scalar>, Xs: Seq<RistrettoPoint>) -> RistrettoPoint {
    let mut acc = IDENTITY_POINT();

    for i in 0..xs.len() {
        acc = add(acc, mul(xs[i], Xs[i]));
    }
    acc
}

// i % 2^(j + 1) < 2^j
fn g(i: usize, j: usize) -> bool {
    1 & (i >> j) == 0
}

// === External Functions === //

/// Create an inner product proof.
/// Note that `G_factors`, and `H_factors` simply represent factors
/// multiplied to the `G` and `H` Seqs respectively.
pub fn create(
    mut transcript: Transcript,
    Q: RistrettoPoint,
    G_factors: Seq<Scalar>,
    H_factors: Seq<Scalar>,
    mut G: Seq<RistrettoPoint>,
    mut H: Seq<RistrettoPoint>,
    mut a: Seq<Scalar>,
    mut b: Seq<Scalar>,
) -> IppRes {
    let mut ret = IppRes::Err(0);
    let mut n = G.len();

    // Check inputs
    if n == H.len()
        && n == a.len()
        && n == b.len()
        && n == G_factors.len()
        && n == H_factors.len()
        && n.is_power_of_two()
    {
        transcript = innerproduct_domain_sep(transcript, U64::classify(n as u64));

        let lg_n = n.log2() as usize;
        let mut L_vec = Seq::<RistrettoPointEncoded>::with_capacity(lg_n);
        let mut R_vec = Seq::<RistrettoPointEncoded>::with_capacity(lg_n);

        // Apply H_factors and G_factors
        for i in 0..n {
            H[i] = mul(H_factors[i], H[i]);
            G[i] = mul(G_factors[i], G[i]);
        }

        // while n != 1
        for _ in 0..lg_n {
            n = n / 2;
            let (mut a_L, a_R) = a.clone().split_off(n);
            let (mut b_L, b_R) = b.clone().split_off(n);
            let (mut G_L, G_R) = G.clone().split_off(n);
            let (mut H_L, H_R) = H.clone().split_off(n);

            let c_L = inner_product(a_L.clone(), b_R.clone());
            let c_R = inner_product(a_R.clone(), b_L.clone());

            // concat(a_L, b_R, c_L), concat(G_R, H_L, Q)
            let L_scalars = (a_L.concat(&b_R)).push(&c_L);
            let L_points = (G_R.concat(&H_L)).push(&Q);

            // concat(a_R, b_L, c_R), concat(G_L, H_R, Q)
            let R_scalars = (a_R.concat(&b_L)).push(&c_R);
            let R_points = (G_L.concat(&H_R)).push(&Q);

            let L = encode(point_dot(L_scalars, L_points));
            let R = encode(point_dot(R_scalars, R_points));

            L_vec = L_vec.push(&L);
            R_vec = R_vec.push(&R);

            transcript = append_point(transcript, L_U8(), L);
            transcript = append_point(transcript, R_U8(), R);

            let (trs, u) = challenge_scalar(transcript, u_U8());
            transcript = trs;
            let u_inv = u.inv();

            for i in 0..n {
                a_L[i] = a_L[i] * u + u_inv * a_R[i];
                b_L[i] = b_L[i] * u_inv + u * b_R[i];
                G_L[i] = add(mul(u_inv, G_L[i]), mul(u, G_R[i]));
                H_L[i] = add(mul(u, H_L[i]), mul(u_inv, H_R[i]));
            }

            a = a_L;
            b = b_L;
            G = G_L;
            H = H_L;
        }

        let ipp = (a[0], b[0], L_vec, R_vec);
        ret = IppRes::Ok((transcript, ipp));
    } else {
        // Handle errors
        if !n.is_power_of_two() {
            ret = IppRes::Err(N_IS_NOT_POWER_OF_TWO);
        } else {
            ret = IppRes::Err(INPUTS_NOT_LEN_N);
        }
    }
    ret
}

/// Returns (u^2, s, u^-2) to be used in the verification algorithm.
/// It is public as it can be faster and more convenient to simply get
/// these values and do the verification outside this module.
pub fn verification_scalars(
    ipp: InnerProductProof,
    n: usize,
    mut transcript: Transcript,
) -> VerScalarsRes {
    let mut res = VerScalarsRes::Err(0);
    let (_a, _b, L_vec, R_vec) = ipp;
    let lg_n = L_vec.len();

    if lg_n >= 32 || n != (1 << lg_n) {
        res = VerScalarsRes::Err(VERIFICATION_ERROR);
    } else {
        transcript = innerproduct_domain_sep(transcript, U64::classify(n as u64));

        // 1. Recompute u_k,...,u_1 based on the proof transcript

        let mut u = Seq::<Scalar>::new(lg_n);
        for i in 0..lg_n {
            transcript = validate_and_append_point(transcript, L_U8(), L_vec[i])?;
            transcript = validate_and_append_point(transcript, R_U8(), R_vec[i])?;
            let (t, c) = challenge_scalar(transcript, u_U8());
            transcript = t;
            u[i] = c;
        }

        // 2. Compute 1/u_i

        let mut u_inv = Seq::<Scalar>::new(lg_n);
        for i in 0..lg_n {
            u_inv[i] = u[i].inv();
        }

        //3. Compute s values

        let mut s = Seq::<Scalar>::new(n);
        for i in 0..n {
            let mut s_i = Scalar::ONE();
            for j in 0..lg_n {
                if g(i, j) {
                    s_i = s_i * u_inv[lg_n - j - 1];
                } else {
                    s_i = s_i * u[lg_n - j - 1];
                }
            }
            s[i] = s_i;
        }

        // 4. Compute u_i^2 and (1/u_i)^2

        for i in 0..lg_n {
            u[i] = u[i].pow(2u128);
            u_inv[i] = u_inv[i].pow(2u128);
        }

        let u_sq = u;
        let u_inv_sq = u_inv;

        res = VerScalarsRes::Ok((u_sq, u_inv_sq, s))
    }

    res
}

/// Checks if a given InnerProductProof and P have been honestly
/// constructed. Returns an error if the verification check fails.
pub fn verify(
    ipp: InnerProductProof,
    n: usize,
    transcript: Transcript,
    G_factors: Seq<Scalar>,
    H_factors: Seq<Scalar>,
    P: RistrettoPoint,
    Q: RistrettoPoint,
    G: Seq<RistrettoPoint>,
    H: Seq<RistrettoPoint>,
) -> VerifyRes {
    let (a, b, L_vec, R_vec) = ipp;
    let (u_sq, u_inv_sq, s) =
        verification_scalars((a, b, L_vec.clone(), R_vec.clone()), n, transcript)?;

    let mut gas = Seq::<Scalar>::new(G.len());
    for i in 0..G.len() {
        gas[i] = G_factors[i] * a * s[i]
    }

    let mut hb_div_s = Seq::<Scalar>::new(H_factors.len());
    for i in 0..H_factors.len() {
        hb_div_s[i] = b * s[i].inv() * H_factors[i]
    }

    let mut neg_u_sq = Seq::<Scalar>::new(u_sq.len());
    let mut neg_u_inv_sq = Seq::<Scalar>::new(u_inv_sq.len());
    for i in 0..u_sq.len() {
        neg_u_sq[i] = Scalar::ZERO() - u_sq[i];
        neg_u_inv_sq[i] = Scalar::ZERO() - u_inv_sq[i];
    }

    let mut Ls = Seq::<RistrettoPoint>::new(L_vec.len());
    let mut Rs = Seq::<RistrettoPoint>::new(R_vec.len());

    for i in 0..Ls.len() {
        Ls[i] = decode(L_vec[i]).or(DecodeRes::Err(VERIFICATION_ERROR))?;
        Rs[i] = decode(R_vec[i]).or(DecodeRes::Err(VERIFICATION_ERROR))?;
    }

    let R_1 = mul(a * b, Q);
    let R_2 = add(point_dot(gas, G), R_1);
    let R_3 = add(point_dot(hb_div_s, H), R_2);
    let R_4 = add(point_dot(neg_u_sq, Ls), R_3);
    let expect_P = add(point_dot(neg_u_inv_sq, Rs), R_4);

    if equals(P, expect_P) {
        VerifyRes::Ok(())
    } else {
        VerifyRes::Err(VERIFICATION_ERROR)
    }
}
