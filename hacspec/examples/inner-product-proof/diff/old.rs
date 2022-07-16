#![allow(non_snake_case)]

#![doc(include = "../docs/inner-product-protocol.md")]

use std::borrow::Borrow;
use std::iter;

use curve25519_dalek::ristretto;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;

use proof_transcript::ProofTranscript;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InnerProductProof {
    pub(crate) L_vec: Vec<RistrettoPoint>,
    pub(crate) R_vec: Vec<RistrettoPoint>,
    pub(crate) a: Scalar,
    pub(crate) b: Scalar,
}

impl InnerProductProof {
    /// Create an inner-product proof.
    ///
    /// The proof is created with respect to the bases \\(G\\), \\(H'\\),
    /// where \\(H'\_i = H\_i \cdot \texttt{Hprime\\_factors}\_i\\).
    ///
    /// The `verifier` is passed in as a parameter so that the
    /// challenges depend on the *entire* transcript (including parent
    /// protocols).
    pub fn create<I>(
        verifier: &mut ProofTranscript,
        Q: &RistrettoPoint,
        Hprime_factors: I,
        mut G_vec: Vec<RistrettoPoint>,
        mut H_vec: Vec<RistrettoPoint>,
        mut a_vec: Vec<Scalar>,
        mut b_vec: Vec<Scalar>,
    ) -> InnerProductProof
    where
        I: IntoIterator,
        I::Item: Borrow<Scalar>,
    {
        // Create slices G, H, a, b backed by their respective
        // vectors.  This lets us reslice as we compress the lengths
        // of the vectors in the main loop below.
        let mut G = &mut G_vec[..];
        let mut H = &mut H_vec[..];
        let mut a = &mut a_vec[..];
        let mut b = &mut b_vec[..];

        let mut n = G.len();

        // All of the input vectors must have the same length.
        assert_eq!(G.len(), n);
        assert_eq!(H.len(), n);
        assert_eq!(a.len(), n);
        assert_eq!(b.len(), n);

        // XXX save these scalar mults by unrolling them into the
        // first iteration of the loop below
        for (H_i, h_i) in H.iter_mut().zip(Hprime_factors.into_iter()) {
            *H_i = (&*H_i) * h_i.borrow();
        }

        let lg_n = n.next_power_of_two().trailing_zeros() as usize;
        let mut L_vec = Vec::with_capacity(lg_n);
        let mut R_vec = Vec::with_capacity(lg_n);

        while n != 1 {
            n = n / 2;
            let (a_L, a_R) = a.split_at_mut(n);
            let (b_L, b_R) = b.split_at_mut(n);
            let (G_L, G_R) = G.split_at_mut(n);
            let (H_L, H_R) = H.split_at_mut(n);

            let c_L = inner_product(&a_L, &b_R);
            let c_R = inner_product(&a_R, &b_L);

            let L = ristretto::vartime::multiscalar_mul(
                a_L.iter().chain(b_R.iter()).chain(iter::once(&c_L)),
                G_R.iter().chain(H_L.iter()).chain(iter::once(Q)),
            );

            let R = ristretto::vartime::multiscalar_mul(
                a_R.iter().chain(b_L.iter()).chain(iter::once(&c_R)),
                G_L.iter().chain(H_R.iter()).chain(iter::once(Q)),
            );

            L_vec.push(L);
            R_vec.push(R);

            verifier.commit(L.compress().as_bytes());
            verifier.commit(R.compress().as_bytes());

            let u = verifier.challenge_scalar();
            let u_inv = u.invert();

            for i in 0..n {
                a_L[i] = a_L[i] * u + u_inv * a_R[i];
                b_L[i] = b_L[i] * u_inv + u * b_R[i];
                G_L[i] = ristretto::vartime::multiscalar_mul(&[u_inv, u], &[G_L[i], G_R[i]]);
                H_L[i] = ristretto::vartime::multiscalar_mul(&[u, u_inv], &[H_L[i], H_R[i]]);
            }

            a = a_L;
            b = b_L;
            G = G_L;
            H = H_L;
        }

        return InnerProductProof {
            L_vec: L_vec,
            R_vec: R_vec,
            a: a[0],
            b: b[0],
        };
    }

    /// Computes three vectors of verification scalars \\([u\_{i}^{2}]\\), \\([u\_{i}^{-2}]\\) and \\([s\_{i}]\\) for combined multiscalar multiplication
    /// in a parent protocol. See [inner product protocol notes](index.html#verification-equation) for details.
    pub(crate) fn verification_scalars(
        &self,
        transcript: &mut ProofTranscript,
    ) -> (Vec<Scalar>, Vec<Scalar>, Vec<Scalar>) {
        let lg_n = self.L_vec.len();
        let n = 1 << lg_n;

        // 1. Recompute x_k,...,x_1 based on the proof transcript

        let mut challenges = Vec::with_capacity(lg_n);
        for (L, R) in self.L_vec.iter().zip(self.R_vec.iter()) {
            // XXX maybe avoid this compression when proof ser/de is sorted out
            transcript.commit(L.compress().as_bytes());
            transcript.commit(R.compress().as_bytes());

            challenges.push(transcript.challenge_scalar());
        }

        // 2. Compute 1/(u_k...u_1) and 1/u_k, ..., 1/u_1

        let mut challenges_inv = challenges.clone();
        let allinv = Scalar::batch_invert(&mut challenges_inv);

        // 3. Compute u_i^2 and (1/u_i)^2

        for i in 0..lg_n {
            // XXX missing square fn upstream
            challenges[i] = challenges[i] * challenges[i];
            challenges_inv[i] = challenges_inv[i] * challenges_inv[i];
        }
        let challenges_sq = challenges;
        let challenges_inv_sq = challenges_inv;

        // 4. Compute s values inductively.

        let mut s = Vec::with_capacity(n);
        s.push(allinv);
        for i in 1..n {
            let lg_i = (32 - 1 - (i as u32).leading_zeros()) as usize;
            let k = 1 << lg_i;
            // The challenges are stored in "creation order" as [u_k,...,u_1],
            // so u_{lg(i)+1} = is indexed by (lg_n-1) - lg_i
            let u_lg_i_sq = challenges_sq[(lg_n - 1) - lg_i];
            s.push(s[i - k] * u_lg_i_sq);
        }

        (challenges_sq, challenges_inv_sq, s)
    }

    /// This method is for testing that proof generation work,
    /// but for efficiency the actual protocols would use `verification_scalars`
    /// method to combine inner product verification with other checks
    /// in a single multiscalar multiplication.
    #[allow(dead_code)]
    pub fn verify<I>(
        &self,
        transcript: &mut ProofTranscript,
        Hprime_factors: I,
        P: &RistrettoPoint,
        Q: &RistrettoPoint,
        G: &[RistrettoPoint],
        H: &[RistrettoPoint],
    ) -> Result<(), ()>
    where
        I: IntoIterator,
        I::Item: Borrow<Scalar>,
    {
        let (u_sq, u_inv_sq, s) = self.verification_scalars(transcript);

        let a_times_s = s.iter().map(|s_i| self.a * s_i);

        // 1/s[i] is s[!i], and !i runs from n-1 to 0 as i runs from 0 to n-1
        let inv_s = s.iter().rev();

        let h_times_b_div_s = Hprime_factors
            .into_iter()
            .zip(inv_s)
            .map(|(h_i, s_i_inv)| (self.b * s_i_inv) * h_i.borrow());

        let neg_u_sq = u_sq.iter().map(|ui| -ui);
        let neg_u_inv_sq = u_inv_sq.iter().map(|ui| -ui);

        let expect_P = ristretto::vartime::multiscalar_mul(
            iter::once(self.a * self.b)
                .chain(a_times_s)
                .chain(h_times_b_div_s)
                .chain(neg_u_sq)
                .chain(neg_u_inv_sq),
            iter::once(Q)
                .chain(G.iter())
                .chain(H.iter())
                .chain(self.L_vec.iter())
                .chain(self.R_vec.iter()),
        );

        if expect_P == *P {
            Ok(())
        } else {
            Err(())
        }
    }
}

/// Computes an inner product of two vectors
/// \\[
///    {\langle {\mathbf{a}}, {\mathbf{b}} \rangle} = \sum\_{i=0}^{n-1} a\_i \cdot b\_i.
/// \\]
/// Panics if the lengths of \\(\mathbf{a}\\) and \\(\mathbf{b}\\) are not equal.
pub fn inner_product(a: &[Scalar], b: &[Scalar]) -> Scalar {
    let mut out = Scalar::zero();
    if a.len() != b.len() {
        panic!("inner_product(a,b): lengths of vectors do not match");
    }
    for i in 0..a.len() {
        out += a[i] * b[i];
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::OsRng;
    use sha2::Sha512;
    use util;

    fn test_helper_create(n: usize) {
        let mut rng = OsRng::new().unwrap();

        use generators::{Generators, PedersenGenerators};
        let gens = Generators::new(PedersenGenerators::default(), n, 1);
        let G = gens.share(0).G.to_vec();
        let H = gens.share(0).H.to_vec();

        // Q would be determined upstream in the protocol, so we pick a random one.
        let Q = RistrettoPoint::hash_from_bytes::<Sha512>(b"test point");

        // a and b are the vectors for which we want to prove c = <a,b>
        let a: Vec<_> = (0..n).map(|_| Scalar::random(&mut rng)).collect();
        let b: Vec<_> = (0..n).map(|_| Scalar::random(&mut rng)).collect();
        let c = inner_product(&a, &b);

        // y_inv is (the inverse of) a random challenge
        let y_inv = Scalar::random(&mut rng);

        // P would be determined upstream, but we need a correct P to check the proof.
        //
        // To generate P = <a,G> + <b,H'> + <a,b> Q, compute
        //             P = <a,G> + <b',H> + <a,b> Q,
        // where b' = b \circ y^(-n)
        let b_prime = b.iter().zip(util::exp_iter(y_inv)).map(|(bi, yi)| bi * yi);
        // a.iter() has Item=&Scalar, need Item=Scalar to chain with b_prime
        let a_prime = a.iter().cloned();

        let P = ristretto::vartime::multiscalar_mul(
            a_prime.chain(b_prime).chain(iter::once(c)),
            G.iter().chain(H.iter()).chain(iter::once(&Q)),
        );

        let mut verifier = ProofTranscript::new(b"innerproducttest");
        let proof = InnerProductProof::create(
            &mut verifier,
            &Q,
            util::exp_iter(y_inv),
            G.clone(),
            H.clone(),
            a.clone(),
            b.clone(),
        );

        let mut verifier = ProofTranscript::new(b"innerproducttest");
        assert!(
            proof
                .verify(&mut verifier, util::exp_iter(y_inv), &P, &Q, &G, &H)
                .is_ok()
        );
    }

    #[test]
    fn make_ipp_1() {
        test_helper_create(1);
    }

    #[test]
    fn make_ipp_2() {
        test_helper_create(2);
    }

    #[test]
    fn make_ipp_4() {
        test_helper_create(4);
    }

    #[test]
    fn make_ipp_32() {
        test_helper_create(32);
    }

    #[test]
    fn make_ipp_64() {
        test_helper_create(64);
    }

    #[test]
    fn test_inner_product() {
        let a = vec![
            Scalar::from_u64(1),
            Scalar::from_u64(2),
            Scalar::from_u64(3),
            Scalar::from_u64(4),
        ];
        let b = vec![
            Scalar::from_u64(2),
            Scalar::from_u64(3),
            Scalar::from_u64(4),
            Scalar::from_u64(5),
        ];
        assert_eq!(Scalar::from_u64(40), inner_product(&a, &b));
    }
}

