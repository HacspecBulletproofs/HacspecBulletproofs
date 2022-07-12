//TODO: Rewrite seqs to vectors/matrices

#![feature(int_log)]
#![allow(non_snake_case)]

use hacspec_lib::*;
use hacspec_linalg_field::*;
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

// asci representations of: L, R, u
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

fn inner_product(u: Seq<Scalar>, v: Seq<Scalar>) -> Scalar {
    let mut ret = Scalar::ZERO();

    for i in 0..u.len() {
        ret = ret + u[i] * v[i]
    }
    ret
}

fn point_dot(xs: Seq<Scalar>, Ps: Seq<RistrettoPoint>) -> RistrettoPoint {
    let mut acc = IDENTITY_POINT();

    for i in 0..xs.len() {
        acc = add(acc, mul(xs[i], Ps[i]));
    }
    acc
}

fn rev(xs: Seq<Scalar>) -> Seq<Scalar> {
    let mut ys = Seq::<Scalar>::new(xs.len());

    for i in 0..xs.len() {
        ys[i] = xs[xs.len() - 1 - i]
    }
    ys
}

// === External Functions === //

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

    // Handle errors
    if n != H.len() || n != a.len() || n != b.len() || n != G_factors.len() || n != H_factors.len()
    {
        ret = IppRes::Err(INPUTS_NOT_LEN_N);
    } else if !n.is_power_of_two() {
        ret = IppRes::Err(N_IS_NOT_POWER_OF_TWO);
    } else {
        transcript = innerproduct_domain_sep(transcript, U64::classify(n as u64));

        let lg_n = n.log2() as usize;
        let mut L_vec = Seq::<RistrettoPointEncoded>::with_capacity(lg_n);
        let mut R_vec = Seq::<RistrettoPointEncoded>::with_capacity(lg_n);

        H = point_dot(H * H_factors);
        G = point_dot(G * G_factors);

        /*
        if n != 1 {
            n = n / 2;
            let (mut a_L, mut a_R) = vector_split(vector_new(a.clone()), n)?;
            let (mut b_L, mut b_R) = vector_split(vector_new(b.clone()), n)?;
            let (Gf_L, Gf_R) = vector_split(vector_new(G_factors.clone()), n)?;
            let (Hf_L, Hf_R) = vector_split(vector_new(H_factors.clone()), n)?;
            let (mut G_L, G_R) = G.clone().split_off(n);
            let (mut H_L, H_R) = H.clone().split_off(n);

            let a_L_prime = seq(matrix_component_mul(a_L.clone(), Gf_R.clone())?);
            let a_R_prime = seq(matrix_component_mul(a_R.clone(), Gf_L.clone())?);
            let b_L_prime = seq(matrix_component_mul(b_L.clone(), Hf_R.clone())?);
            let b_R_prime = seq(matrix_component_mul(b_R.clone(), Hf_L.clone())?);

            let c_L = vector_dot(a_L.clone(), b_R.clone())?;
            let c_R = vector_dot(a_R.clone(), b_L.clone())?;

            // concat(a_L, b_R, c_L), concat(G_R, H_L, Q)
            let L_scalars = (a_L_prime.concat(&b_R_prime)).push(&c_L);
            let L_points = (G_R.concat(&H_L)).push(&Q);

            // concat(a_R, b_L, c_R), concat(G_L, H_R, Q)
            let R_scalars = (a_R_prime.concat(&b_L_prime)).push(&c_R);
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

            // u     * a_L + u_inv * a_R
            // u_inv * b_L + u     * b_R
            // u_inv * Gf_L * G_L + u     * Gf_R * G_R
            // u     * Hf_L * H_L + u_inv * Hf_R * H_R
            a_L = matrix_add(matrix_scale(a_L, u), matrix_scale(a_R, u_inv))?;
            b_L = matrix_add(matrix_scale(b_L, u_inv), matrix_scale(b_R, u))?;
            for i in 0..n {
                G_L[i] = add(
                    mul(u_inv * seq(Gf_L.clone())[i], G_L[i]),
                    mul(u * seq(Gf_R.clone())[i], G_R[i]),
                );
                H_L[i] = add(
                    mul(u * seq(Hf_L.clone())[i], H_L[i]),
                    mul(u_inv * seq(Hf_R.clone())[i], H_R[i]),
                );
            }

            a = seq(a_L);
            b = seq(b_L);
            G = G_L;
            H = H_L;
        }
        */

        // while n != 1
        for _ in 1..lg_n {
            n = n / 2;
            let (mut a_L, a_R) = a.clone().split_off(n);
            let (mut b_L, b_R) = b.clone().split_off(n);
            let (mut G_L, G_R) = G.clone().split_off(n);
            let (mut H_L, H_R) = H.clone().split_off(n);

            let c_L = inner_product(a_L.clone(), b_R.clone());
            let c_R = inner_product(a_R.clone(), b_L.clone());

            let La = point_dot(a_L.clone(), G_R.clone());
            let Lb = point_dot(b_R.clone(), H_L.clone());
            let Lc = mul(c_L, Q);

            let Ra = point_dot(a_R.clone(), G_L.clone());
            let Rb = point_dot(b_L.clone(), H_R.clone());
            let Rc = mul(c_R, Q);

            let L = encode(add(add(La, Lb), Lc));
            let R = encode(add(add(Ra, Rb), Rc));

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
    }

    ret
}

pub fn verification_scalars(
    ipp: InnerProductProof,
    n: usize,
    mut transcript: Transcript,
) -> VerScalarsRes {
    let mut res = VerScalarsRes::Err(0);
    let (a, b, L_vec, R_vec) = ipp;
    let lg_n = L_vec.len();

    if lg_n >= 32 || n != (1 << lg_n) {
        res = VerScalarsRes::Err(VERIFICATION_ERROR);
    } else {
        transcript = innerproduct_domain_sep(transcript, U64::classify(n as u64));

        // 1. Recompute x_k,...,x_1 based on the proof transcript

        let mut challenges = Seq::<Scalar>::new(lg_n);
        for i in 0..lg_n {
            transcript = validate_and_append_point(transcript, L_U8(), L_vec[i])?;
            transcript = validate_and_append_point(transcript, R_U8(), R_vec[i])?;
            let (t, c) = challenge_scalar(transcript, u_U8());
            transcript = t;
            challenges[i] = c;
        }

        // 2. Compute 1/(u_k * ... * u_1)

        let mut challenges_inv = challenges.clone();
        // allinv = u[0] * ... * u[n]
        let mut allinv = Scalar::ONE();
        for i in 0..lg_n {
            challenges_inv[i] = challenges_inv[i].inv();
            allinv = allinv * challenges_inv[i];
        }

        // 3. Compute u_i^2 and (1/u_i)^2

        for i in 0..lg_n {
            challenges[i] = challenges[i].pow(2u128);
            challenges_inv[i] = challenges_inv[i].pow(2u128);
        }

        let mut challenges_sq = challenges;
        let mut challenges_inv_sq = challenges_inv;

        //4. Compute s values inductively

        let mut s = Seq::<Scalar>::with_capacity(n);
        s = s.push(&allinv);
        for i in 1..n {
            let lg_i = (32u32 - 1u32 - (i as u32).leading_zeros()) as usize;
            let k = 1 << lg_i;
            let u_lg_i_sq = challenges_sq[(lg_n - 1) - lg_i];
            s = s.push(&(s[i - k] * u_lg_i_sq));
        }

        res = VerScalarsRes::Ok((challenges_sq, challenges_inv_sq, s))
    }

    res
}

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
        gas[i] = a * s[i] * G_factors[i]
    }

    let inv_s = rev(s);

    let mut hb_div_s = Seq::<Scalar>::new(H_factors.len());
    for i in 0..H_factors.len() {
        hb_div_s[i] = b * inv_s[i] * H_factors[i]
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
