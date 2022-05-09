//TODO: Rewrite seqs to vectors/matrices
#![feature(int_log)]
use hacspec_lib::*;
use hacspec_ristretto::*;
use hacspec_ristretto as ristretto;

use hacspec_linalg_field::*;
use hacspec_linalg_field as linalg;

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

//We need to generate better Base Points
fn point_dot(v: Seq::<FieldElement>, p: Seq::<RistrettoPoint>) -> RistrettoPoint {
	let mut acc = IDENTITY_POINT();
	for i in 0..v.len() {
		acc = ristretto::add(acc, ristretto::mul(v[i], p[i]));
	}
	acc
}

pub fn create(
	transscript: String,
	Q: RistrettoPoint,
	G_factors: Seq<FieldElement>,
	H_factors: Seq<FieldElement>,
	G: Seq<RistrettoPoint>,
	H: Seq<RistrettoPoint>,
	a: Seq<FieldElement>,
	b: Seq<FieldElement>,
) -> Result<(), ()> {
	let mut ret = Result::<(), ()>::Err(());
	let mut n = G.len();

	if n.is_power_of_two()
		&& n == H.len()
		&& n == a.len()
		&& n == b.len()
		&& n == G_factors.len()
		&& n == H_factors.len()
		&& n.is_power_of_two()
	{
		let lg_n = n.log2() as usize;
		let mut L_vec = linalg::zeros(lg_n, 1);
		let mut R_vec = linalg::zeros(lg_n, 1);

		while n != 1 {
			n = n / 2;
			let (a_L, a_R) = a.clone().split_off(n);
			let (b_L, b_R) = b.clone().split_off(n);
			let (G_L, G_R) = G.clone().split_off(n);
			let (H_L, H_R) = H.clone().split_off(n);

			let c_L = inner_product(a_L.clone(), b_R.clone());
			let c_R = inner_product(a_R.clone(), b_L.clone());

			let La = point_dot(a_L, G_R);
			let Lb = point_dot(b_R, H_L);
			let Lc = ristretto::mul(c_L, Q);

			let Ra = point_dot(a_R, G_L);
			let Rb = point_dot(b_L, H_R);
			let Rc = ristretto::mul(c_R, Q);

			let L = ristretto::add(ristretto::add(La, Lb), Lc);
			let R = ristretto::add(ristretto::add(Ra, Rb), Rc);

			break;
		}

		ret = Result::<(), ()>::Ok(());
	}

	ret

	//transscript.append("innerproduct_domain_sep(n)")
}
