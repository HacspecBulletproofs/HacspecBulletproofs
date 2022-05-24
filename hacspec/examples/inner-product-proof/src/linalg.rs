//TODO: Rewrite seqs to vectors/matrices
#![feature(int_log)]
mod transcript;

use hacspec_lib::*;

use hacspec_ristretto::*;
use hacspec_merlin::*;
use crate::transcript::*;

use hacspec_ristretto as ristretto;
use hacspec_linalg_field as linalg;

pub type InnerProductProof = (Scalar, Scalar, Seq::<RistrettoPoint>, Seq::<RistrettoPoint>);

fn print_point(label: &str, P: RistrettoPoint) {
	print!("{}: [", label);
	let bytes = encode(P).to_le_bytes();
	for i in 0..bytes.len() {
		print!("{}, ", bytes[i].declassify());
	}
	print!("]\n");
	println!("");
}

fn print_fe(label: &str, f: Scalar) {
	print!("{}: [", label);
	let bytes = f.to_byte_seq_le();
	for i in 0..bytes.len() {
		print!("{}, ", bytes[i]);
	}
	print!("]\n");
	println!("");
}

fn print_state(label: &str, f: hacspec_merlin::strobe::StateU8) {
	print!("{}: [", label);
	let bytes = f;
	for i in 0..bytes.len() {
		print!("{}, ", bytes[i]);
	}
	print!("]\n");
	println!("");
}

fn inner_product(u: Seq::<Scalar>, v: Seq::<Scalar>) -> Scalar {
	let mut ret = Scalar::ZERO();
	if u.len() != v.len() {
		panic!("{},{}", u.len(), v.len());
	}
	for i in 0..u.len()-1 {
		ret = ret + u[i] + v[i]
	}
	ret
}

fn point_dot(v: Seq::<Scalar>, p: Seq::<RistrettoPoint>) -> RistrettoPoint {
	let mut acc = IDENTITY_POINT();
	for i in 0..v.len() {
		acc = ristretto::add(acc, ristretto::mul(v[i], p[i]));
	}
	acc
}

pub fn create(
	mut transcript: Transcript,
	Q: RistrettoPoint,
	G_factors: Seq<Scalar>,
	H_factors: Seq<Scalar>,
	G: Seq<RistrettoPoint>,
	H: Seq<RistrettoPoint>,
	a: Seq<Scalar>,
	b: Seq<Scalar>,
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
		print_state("t0", transcript.0);
		transcript = innerproduct_domain_sep(transcript, U64::classify(n as u64));
		print_state("t1", transcript.0);

		let lg_n = n.log2() as usize;
		let mut L_vec = Seq::<RistrettoPoint>::new(lg_n);
		let mut R_vec = Seq::<RistrettoPoint>::new(lg_n);

		while n != 1 {
			println!("n: {}", n);
			n = n / 2;
			let (a_L, a_R) = a.clone().split_off(n);
			let (b_L, b_R) = b.clone().split_off(n);
			let (G_L, G_R) = G.clone().split_off(n);
			let (H_L, H_R) = H.clone().split_off(n);

			let c_L = inner_product(a_L.clone(), b_R.clone());
			let c_R = inner_product(a_R.clone(), b_L.clone());

			print_fe("c_L", c_L);
			print_fe("c_R", c_R);

			let La = point_dot(a_L.clone(), G_R.clone());
			let Lb = point_dot(b_R.clone(), H_L.clone());
			let Lc = ristretto::mul(c_L, Q);

			let Ra = point_dot(a_R.clone(), G_L.clone());
			let Rb = point_dot(b_L.clone(), H_R.clone());
			let Rc = ristretto::mul(c_R, Q);

			let L = ristretto::add(ristretto::add(La, Lb), Lc);
			let R = ristretto::add(ristretto::add(Ra, Rb), Rc);

			print_point("L:", L);
			print_point("R:", R);

			L_vec.push(&L);
			R_vec.push(&R);

			transcript = append_point(transcript, byte_seq!(76u8), ristretto::encode(L));
			print_state("state L", transcript.0);
			transcript = append_point(transcript, byte_seq!(82u8), ristretto::encode(R));
			print_state("state R", transcript.0);

			let (trs, u) = challenge_scalar(transcript, byte_seq!(117u8));
			transcript = trs;
			let u_inv = u.inv();

			print_fe("u", u);

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

			println!("");
		}

		ret = Result::<InnerProductProof, ()>::Ok((a[0], b[0], G, H));
	}

	ret

	//transscript.append("innerproduct_domain_sep(n)")
}
