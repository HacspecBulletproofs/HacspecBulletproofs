pub fn mul2(m1: Matrix, m2: Matrix) -> Result<Matrix, ()> {
	let (dim_1, seq_1) = m1;
	let (dim_2, seq_2) = m2;
	let (m, n) = dim_1;
	let (n_2, p) = dim_2;
	let mut ret = Seq::<Scalar>::new(m*p);
	let mut res = Result::<Matrix, ()>::Err(());

	if n == n_2 {
		for i in 0..m {
			for j in 0..p {
				let mut acc = Scalar::ZERO();
				let index = i * p + j;

				for k in 0..n {
					let index_1 = i * n + k;
					let index_2 = k * p + j;
					println!("index = {}", index);
					println!("seq_1[{}] = {}, seq_2[{}] = {}", index_1, seq_1[index_1], index_2, seq_2[index_2]);

					acc = acc + seq_1[index_1] * seq_2[index_2];
					println!("acc = {}", acc);
					println!("");
				}

				ret[index] = acc
			}
		}

		res = Result::<Matrix, ()>::Ok(new(n, p, ret).unwrap())
	}

	res
}

use hacspec_lib::prelude::*;
use hacspec_linalg::*; 

fn main() -> Result<(),()> {
	let xs = vec![
		1,2,1,
		0,1,1];
	let ys = vec![
		2,5,1,1,
		6,7,1,1,
		1,1,1,1];
	let zs = vec![
		15,20,4,4,
		7, 8, 2,2];

	println!("1");
	let hac_xs = new(2, 3, Seq::<Scalar>::from_vec(xs.clone()))?;
	println!("2");
	let hac_ys = new(3, 4, Seq::<Scalar>::from_vec(ys.clone()))?;
	println!("3");
	let hac_zs = new(2, 4, Seq::<Scalar>::from_vec(zs.clone()))?;
	println!("4");
	println!("({},{}), ({},{})", hac_xs.0.0, hac_xs.0.1, hac_ys.0.0, hac_ys.0.1);

	let hac_mul = mul2(hac_xs, hac_ys)?;
	println!("5");
	println!("6: {:?}", hac_mul.1.native_slice());

	Ok(())
}
