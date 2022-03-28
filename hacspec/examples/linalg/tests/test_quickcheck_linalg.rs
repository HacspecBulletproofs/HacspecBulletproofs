extern crate quickcheck;
extern crate nalgebra;

use hacspec_lib::prelude::*;
use hacspec_linalg::*; 

use nalgebra::DMatrix;
use quickcheck::*;

// === Helper functions ===

fn assert_matrices(x: Matrix, y: DMatrix<Scalar>) -> bool {
	if x.0 != (y.nrows(), y.ncols()) {
		false
	} else {
		let y = y.transpose();
		let zipped = x.1.iter().zip(y.iter());

		for z in zipped {
			if z.0 != z.1 {
				panic!("({:?}) == ({:?}), ({},{}) == ({},{})", x.1.native_slice(), y.as_slice(), x.0.0, x.0.1, y.nrows(), y.ncols());
			}
		}

		true
	}
}

fn quickcheck(helper: impl Testable) {
	QuickCheck::new()
		.tests(50)
		.min_tests_passed(50)
		.max_tests(100000000000)
		.quickcheck(helper);
}

fn bv(xs:Vec<i128>) -> Vec<BigInt> {
	xs.into_iter().map(|x| BigInt::from(x)).collect()
}

fn dmatrix(n:usize, m:usize, xs:Vec<BigInt>) -> DMatrix<BigInt> {
	DMatrix::from_vec(m, n, xs).transpose()
}

// === Tests ===

#[test]
fn test_index() {
	fn helper(xs:Vec<i128>, n:u8, m:u8) -> TestResult {
		let mut xs = bv(xs);
		let n = n as usize;
		let m = m as usize;

		if n*m == 0 || n*m > xs.len() {
			return TestResult::discard()
		}

		xs.truncate(n*m);

		let hac = new(n, m, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
		let ext = dmatrix(n, m, xs.clone());

		let mut eq = true;
		for i in 0..n {
			for j in 0..m {
				let hac_op = index(i, j, hac.clone()).unwrap();
				let ext_op = ext.index((i, j));

				if hac_op != *ext_op {
					eq = false
				}
			}
		}
	
		TestResult::from_bool(eq)
	}
	quickcheck(helper as fn(Vec<i128>, u8, u8) -> TestResult);
}

#[test]
fn test_transpose() {
	fn helper(xs:Vec<i128>, n:u8, m:u8) -> TestResult {
		let mut xs = bv(xs);
		let n = n as usize;
		let m = m as usize;

		if n*m == 0 || n*m > xs.len() {
			return TestResult::discard()
		}

		xs.truncate(n*m);

		let hac = new(n, m, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
		let ext = dmatrix(n, m, xs.clone());

		let hac_op = transpose(hac);
		let ext_op = ext.transpose();
	
		TestResult::from_bool(assert_matrices(hac_op, ext_op))
	}
	quickcheck(helper as fn(Vec<i128>, u8, u8) -> TestResult);
}

#[test]
fn test_zeros() {
	fn helper(n:u8, m:u8) -> TestResult {
		let n = n as usize;
		let m = m as usize;

		if n*m == 0 {
			return TestResult::discard()
		}

		let hac = zeros(n,m).unwrap();
		let ext = DMatrix::zeros(n,m);

		TestResult::from_bool(assert_matrices(hac, ext))
	}
	quickcheck(helper as fn(u8, u8) -> TestResult);
}

#[test]
fn test_identity() {
	fn helper(n:u8) -> TestResult {
		let n = n as usize;

		if n == 0 {
			return TestResult::discard()
		}

		let hac = identity(n).unwrap();
		let ext = DMatrix::identity(n,n);

		TestResult::from_bool(assert_matrices(hac, ext))
	}
	quickcheck(helper as fn(u8) -> TestResult);
}

#[test]
fn test_slice() {
	fn helper(xs:Vec<i128>, n:u8, m:u8) -> TestResult {
		let mut xs = bv(xs);
		let n = n as usize;
		let m = m as usize;

		if n*m == 0 || n*m > xs.len() {
			return TestResult::discard()
		}

		xs.truncate(n*m);

		let hac = new(n, m, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
		let ext = dmatrix(n, m, xs.clone());

		let mut eq = true;
		for i in 0..n-1 {
			for j in 0..m-1 {
				for k in 1..n-i-1 {
					for l in 1..m-j-1 {
						let hac_op = slice((i,j), k,l, hac.clone()).unwrap();
						let ext_op: DMatrix<BigInt> = ext.slice((i,j), (k,l)).into();

						if !assert_matrices(hac_op.clone(), ext_op.clone()) {
							eq = false
						}
					}
				}
			}
		}

		TestResult::from_bool(eq)
	}
	quickcheck(helper as fn(Vec<i128>, u8, u8) -> TestResult);
}

#[test]
fn test_add() {
	fn helper(xs:Vec<i128>, ys:Vec<i128>, n:u8, m:u8) -> TestResult {
		let mut xs = bv(xs);
		let mut ys = bv(ys);
		let n = n as usize;
		let m = m as usize;

		if n*m == 0 || n*m > xs.len() || n*m > ys.len() {
			return TestResult::discard()
		}

		xs.truncate(n*m);
		ys.truncate(n*m);

		let hac_xs = new(n, m, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
		let hac_ys = new(n, m, Seq::<Scalar>::from_vec(ys.clone())).unwrap();

		let ext_xs = dmatrix(n, m, xs.clone());
		let ext_ys = dmatrix(n, m, ys.clone());

		let hac_op = add(hac_xs, hac_ys).unwrap();
		let ext_op = ext_xs + ext_ys;
	
		TestResult::from_bool(assert_matrices(hac_op, ext_op))
	}
	quickcheck(helper as fn(Vec<i128>, Vec<i128>, u8, u8) -> TestResult);
}

#[test]
fn test_sub() {
	fn helper(xs:Vec<i128>, ys:Vec<i128>, n:u8, m:u8) -> TestResult {
		let mut xs = bv(xs);
		let mut ys = bv(ys);
		let n = n as usize;
		let m = m as usize;

		if n*m == 0 || n*m > xs.len() || n*m > ys.len() {
			return TestResult::discard()
		}

		xs.truncate(n*m);
		ys.truncate(n*m);

		let hac_xs = new(n, m, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
		let hac_ys = new(n, m, Seq::<Scalar>::from_vec(ys.clone())).unwrap();

		let ext_xs = dmatrix(n, m, xs.clone());
		let ext_ys = dmatrix(n, m, ys.clone());


		let hac_op = sub(hac_xs, hac_ys).unwrap();
		let ext_op = ext_xs - ext_ys;
	
		TestResult::from_bool(assert_matrices(hac_op, ext_op))
	}
	quickcheck(helper as fn(Vec<i128>, Vec<i128>, u8, u8) -> TestResult);
}

#[test]
fn test_component_mul() {
	fn helper(xs:Vec<i128>, ys:Vec<i128>, n:u8, m:u8) -> TestResult {
		let mut xs = bv(xs);
		let mut ys = bv(ys);
		let n = n as usize;
		let m = m as usize;

		if n*m == 0 || n*m > xs.len() || n*m > ys.len() {
			return TestResult::discard()
		}

		xs.truncate(n*m);
		ys.truncate(n*m);

		let hac_xs = new(n, m, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
		let hac_ys = new(n, m, Seq::<Scalar>::from_vec(ys.clone())).unwrap();

		let ext_xs = dmatrix(n, m, xs.clone());
		let ext_ys = dmatrix(n, m, ys.clone());


		let hac_op = component_mul(hac_xs, hac_ys).unwrap();
		let ext_op = ext_xs.component_mul(&ext_ys);
	
		TestResult::from_bool(assert_matrices(hac_op, ext_op))
	}
	quickcheck(helper as fn(Vec<i128>, Vec<i128>, u8, u8) -> TestResult);
}

#[test]
fn test_mul() {
	fn helper(xs:Vec<i128>, ys:Vec<i128>, n:u8, m:u8, p:u8) -> TestResult {
		let mut xs = bv(xs);
		let mut ys = bv(ys);
		let n = n as usize;
		let m = m as usize;
		let p = p as usize;

		if n*m*p == 0 || n*m > xs.len() || m*p > ys.len() {
			return TestResult::discard()
		}

		xs.truncate(n*m);
		ys.truncate(m*p);

		let hac_xs = new(n, m, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
		let hac_ys = new(m, p, Seq::<Scalar>::from_vec(ys.clone())).unwrap();

		let ext_xs = dmatrix(n, m, xs.clone());
		let ext_ys = dmatrix(m, p, ys.clone());


		let hac_op = mul(hac_xs, hac_ys).unwrap();
		let ext_op = ext_xs.mul(&ext_ys);
	
		TestResult::from_bool(assert_matrices(hac_op, ext_op))
	}
	quickcheck(helper as fn(Vec<i128>, Vec<i128>, u8, u8, u8) -> TestResult);
}
