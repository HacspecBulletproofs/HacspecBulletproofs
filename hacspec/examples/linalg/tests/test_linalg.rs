extern crate quickcheck;
extern crate nalgebra;

use hacspec_lib::prelude::*;
use hacspec_linalg::*; 

use nalgebra::DMatrix;
use quickcheck::*;

fn assert_matrices(x: Matrix, y: DMatrix<Scalar>) -> bool {
	if x.0 != (y.nrows(), y.ncols()) {
		false
	} else {
		let y = y.transpose();
		let zipped = x.1.iter().zip(y.iter());
		let mut eq = true;

		for z in zipped {
			if z.0 != z.1 {
				eq = false
			}
		}

		eq
	}
}

fn assert_hacs(x: Matrix, y: Matrix) -> bool {
	if x.0 != y.0 {
		false
	} else {
		let zipped = x.1.iter().zip(y.1.iter());
		let mut eq = true;

		for z in zipped {
			if z.0 != z.1 {
				eq = false
			}
		}

		eq
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

//#[test]
//fn test_unit_matrix_mul() {
//	let xs = bv(vec![1,2,1, 0,1,1]);
//	let ys = bv(vec![2,5,1,1, 6,7,1,1, 1,1,1,1]);
//	let zs = bv(vec![15,20,4,4, 7,8,2,2]);
//
//	let hac_xs = new(2, 3, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
//	let hac_ys = new(3, 4, Seq::<Scalar>::from_vec(ys.clone())).unwrap();
//	let hac_zs = new(2, 4, Seq::<Scalar>::from_vec(zs.clone())).unwrap();
//
//	let hac_mul = mul(hac_xs, hac_ys).unwrap();
//
//	assert!(assert_hacs(hac_zs, hac_mul));
//}
//

#[test]
fn test_1() {
	let mut xs = bv(vec![0, 0, 0, 0, 0]);
	let mut ys = bv(vec![0, 0, 0, 0, 0, 0]);
	let n = 1usize;
	let m = 2usize;

	xs.truncate(n*m);
	ys.truncate(n*m);

	let hac_xs = new(n, m, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
	let hac_ys = new(n, m, Seq::<Scalar>::from_vec(ys.clone())).unwrap();

	let ext_xs = dmatrix(n, m, xs.clone());
	let ext_ys = dmatrix(n, m, ys.clone());

	let hac_op = add(hac_xs, hac_ys).unwrap();
	let ext_op = ext_xs + ext_ys;

	println!("start");
	println!("({},{}), ({},{})", hac_op.0.0, hac_op.0.1, ext_op.nrows(), ext_op.ncols());
	println!("({:?})", hac_op.1.native_slice());
	println!("({:?})", ext_op.as_slice());



	assert!(assert_matrices(hac_op, ext_op));
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
fn test_hadamard() {
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


		let hac_op = hadamard(hac_xs, hac_ys).unwrap();
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

////#[test]
//fn test_mul_matrices() {
//	fn helper(xs: Vec<i128>, ys: Vec<i128>, m: usize, n: usize, p: usize) -> TestResult {
//		if xs.len() != m*n {
//			return TestResult::discard()
//		}
//		if ys.len() != n*p {
//			return TestResult::discard()
//		}
//		if xs.len() == 0 && ys.len() == 0 {
//			return TestResult::discard()
//		}
//
//		let hac_xs_res = new(m, n, Seq::<Scalar>::from_vec(xs.clone()));
//		let hac_ys_res = new(n, p, Seq::<Scalar>::from_vec(ys.clone()));
//		if hac_xs_res.is_err() {
//			return TestResult::from_bool(false)
//		}
//		if hac_ys_res.is_err() {
//			return TestResult::from_bool(false)
//		}
//
//		let hac_xs = hac_xs_res.unwrap();
//		let hac_ys = hac_ys_res.unwrap();
//
//		let ext_xs = DMatrix::from_vec(m, n, xs.clone());
//		let ext_ys = DMatrix::from_vec(n, p, ys.clone());
//
//		// Catch panics
//		let ext_res = catch_unwind(|| { let _ = ext_xs.clone().mul(ext_ys.clone()); });
//		let hac_res = catch_unwind(|| { let _ = mul(hac_xs.clone(), hac_ys.clone()).unwrap(); });
//
//		println!("Prepare!");
//		println!("ext_res {}", ext_res.is_ok());
//		println!("hac_res {}", hac_res.is_ok());
//
//		if ext_res.is_ok() && hac_res.is_ok() {
//			println!("Enter!");
//			let hac_op = mul(hac_xs, hac_ys).unwrap();
//			let ext_op = ext_xs * ext_ys;
//	
//			TestResult::from_bool(assert_thing2(hac_op, ext_op))
//		} else if ext_res.is_err() && hac_res.is_err() {
//			TestResult::from_bool(true)
//		} else {
//			TestResult::from_bool(false)
//		}
//	}
//	QuickCheck::new()
//		.tests(1000)
//		.min_tests_passed(1000)
//		.max_tests(100000000000)
//		.quickcheck(helper as fn(Vec<i128>, Vec<i128>, usize, usize, usize) -> TestResult);
//}
