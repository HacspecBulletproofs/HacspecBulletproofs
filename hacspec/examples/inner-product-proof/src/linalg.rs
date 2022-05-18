//TODO: Rewrite seqs to vectors/matrices
#![feature(int_log)]
mod transcript;

use hacspec_lib::*;

use hacspec_ristretto::*;
use hacspec_merlin::*;
use crate::transcript::*;

use hacspec_ristretto as ristretto;
use hacspec_linalg_field as linalg;

pub type InnerProductProof = (FieldElement, FieldElement, Seq::<RistrettoPoint>, Seq::<RistrettoPoint>);

fn inner_product(u: Seq::<FieldElement>, v: Seq::<FieldElement>) -> FieldElement {
	let mut ret = FieldElement::ZERO();
	if u.len() != v.len() {
		panic!("{},{}", u.len(), v.len());
	}
	for i in 0..u.len()-1 {
		ret = ret + u[i] + v[i]
	}
	ret
}

fn point_dot(v: Seq::<FieldElement>, p: Seq::<RistrettoPoint>) -> RistrettoPoint {
	let mut acc = IDENTITY_POINT();
	for i in 0..v.len() {
		acc = ristretto::add(acc, ristretto::mul(v[i], p[i]));
	}
	acc
}

pub fn create(
	mut transcript: Transcript,
	Q: RistrettoPoint,
	G_factors: Seq<FieldElement>,
	H_factors: Seq<FieldElement>,
	G: Seq<RistrettoPoint>,
	H: Seq<RistrettoPoint>,
	a: Seq<FieldElement>,
	b: Seq<FieldElement>,
) -> Result::<InnerProductProof, ()> {
	let mut ret = Result::<InnerProductProof, ()>::Err(());

	let mut G = G;
	let mut H = H;
	let mut a = a;
	let mut b = b;

	let mut n = G.len();

	if n.is_power_of_two()
		&& n == H.len()
		&& n == a.len()
		&& n == b.len()
		&& n == G_factors.len()
		&& n == H_factors.len()
		&& n.is_power_of_two()
	{
		transcript = innerproduct_domain_sep(transcript, U64::classify(n as u64));

		let lg_n = n.log2() as usize;
		let mut L_vec = Seq::<RistrettoPoint>::new(lg_n);
		let mut R_vec = Seq::<RistrettoPoint>::new(lg_n);

		while n != 1 {
			n = n / 2;
			let (a_L, a_R) = a.clone().split_off(n);
			let (b_L, b_R) = b.clone().split_off(n);
			let (G_L, G_R) = G.clone().split_off(n);
			let (H_L, H_R) = H.clone().split_off(n);

			let c_L = inner_product(a_L.clone(), b_R.clone());
			let c_R = inner_product(a_R.clone(), b_L.clone());

			let La = point_dot(a_L.clone(), G_R.clone());
			let Lb = point_dot(b_R.clone(), H_L.clone());
			let Lc = ristretto::mul(c_L, Q);

			let Ra = point_dot(a_R.clone(), G_L.clone());
			let Rb = point_dot(b_L.clone(), H_R.clone());
			let Rc = ristretto::mul(c_R, Q);

			let L = ristretto::add(ristretto::add(La, Lb), Lc);
			let R = ristretto::add(ristretto::add(Ra, Rb), Rc);

			L_vec.push(&L);
			R_vec.push(&R);

			transcript = append_point(transcript, byte_seq!(76u8), ristretto::encode(L));
			transcript = append_point(transcript, byte_seq!(82u8), ristretto::encode(R));
			let (trs, u) = challenge_scalar(transcript, byte_seq!(117u8));
			transcript = trs;
			let u_inv = u.inv();

			let mut a_ = a_L.clone();
			let mut b_ = b_L.clone();
			let mut G_ = G_L.clone();
			let mut H_ = H_L.clone();

			for i in 0..n {
				a_[i] = a_[i] * u + u_inv * a_[i];
				b_[i] = b_[i] * u_inv + u * b_[i];
				G_[i] = add(mul(u, G_[i]), mul(u_inv, G_[i]));
				H_[i] = add(mul(u, H_[i]), mul(u_inv, H_[i]));
			}

			a = a_L;
			b = b_L;
			G = G_L;
			H = H_L;
		}

		ret = Result::<InnerProductProof, ()>::Ok((a[0], b[0], G, H));
	}

	ret

	//transscript.append("innerproduct_domain_sep(n)")
}
