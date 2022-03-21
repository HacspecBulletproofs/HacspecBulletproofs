use hacspec_lib::prelude::*;
use hacspec_linalg::*; 

extern crate quickcheck;
extern crate nalgebra;
use nalgebra::DVector;
use nalgebra::DMatrix;
use std::panic::catch_unwind;
use quickcheck::*;

fn assert_thing(x: Matrix, y: DVector<Scalar>) -> bool {
	let zipped = x.1.iter().zip(y.iter());
	let mut eq = true;
	for z in zipped {
		if z.0 != z.1 {
			eq = false
		}
	}
	eq
}

fn assert_thing2(x: Matrix, y: DMatrix<Scalar>) -> bool {
	let zipped = x.1.iter().zip(y.iter());
	let mut eq = true;
	for z in zipped {
		if z.0 != z.1 {
			eq = false
		}
	}
	eq
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

fn print_shit(xs: &[Scalar]) {
	println!("");
	for i in 0..xs.len() {
		print!("{}, ", xs[i])
	}
}

#[test]
fn test_unit_matrix_mul() -> Result<(),()> {
	let xs = vec![1,2,1, 0,1,1];
	let ys = vec![2,5,1,1, 6,7,1,1, 1,1,1,1];
	let zs = vec![15,20,4,4, 7,8,2,2];

	println!("1");
	let hac_xs = new(2, 3, Seq::<Scalar>::from_vec(xs.clone()))?;
	println!("2");
	let hac_ys = new(3, 4, Seq::<Scalar>::from_vec(ys.clone()))?;
	println!("3");
	let hac_zs = new(2, 4, Seq::<Scalar>::from_vec(zs.clone()))?;
	println!("4");
	println!("({},{}), ({},{})", hac_xs.0.0, hac_xs.0.1, hac_ys.0.0, hac_ys.0.1);

	let hac_mul = mul(hac_xs, hac_ys)?;
	println!("5");

	println!("final: ({},{})", hac_mul.0.0, hac_mul.0.1);
	print_shit(hac_mul.1.native_slice());
	print_shit(hac_zs.1.native_slice());
	assert!(assert_hacs(hac_zs, hac_mul));
	Ok(())
}

//#[test]
fn test_1() -> Result<(),()>{
	let xs = vec![0i128,0i128];
	let ys = vec![0i128,1i128];

	let hac_xs_res = new(xs.len(), 1, Seq::<Scalar>::from_vec(xs.clone()));
	let hac_ys_res = new(ys.len(), 1, Seq::<Scalar>::from_vec(ys.clone()));
	println!("p0");
	let hac_xs = hac_xs_res.unwrap();
	println!("p1");
	let hac_ys = hac_ys_res?;
	println!("p2");

	let ext_xs = DVector::from_vec(xs.clone());
	let ext_ys = DVector::from_vec(ys.clone());

	// Catch panics
	let ext_res = catch_unwind(|| { let _ = ext_xs.clone().add(ext_ys.clone()); });
	let hac_res = catch_unwind(|| { let _ = sub(hac_xs.clone(), hac_ys.clone()); });

	if ext_res.is_ok() && hac_res.is_ok() {
		let hac_op = sub(hac_xs, hac_ys)?;
		let ext_op = ext_xs - ext_ys;

		if (assert_thing(hac_op, ext_op)) == true {
			println!("p3");
			Ok(())
		} else {
			println!("p4");
			Err(())
		}
	} else if ext_res.is_err() && hac_res.is_err() {
		println!("p5");
		Ok(())
	} else {
		println!("p6");
		Err(())
	}
}

//#[test]
fn test_add_vectors() {
	fn helper(xs: Vec<i128>, ys: Vec<i128>) -> TestResult {
		if xs.len() != ys.len() {
			return TestResult::discard()
		}
		if xs.len() == 0 && ys.len() == 0 {
			return TestResult::discard()
		}

		let hac_xs_res = new(xs.len(), 1, Seq::<Scalar>::from_vec(xs.clone()));
		let hac_ys_res = new(ys.len(), 1, Seq::<Scalar>::from_vec(ys.clone()));
		if hac_xs_res.is_err() {
			return TestResult::from_bool(false)
		}
		if hac_ys_res.is_err() {
			return TestResult::from_bool(false)
		}

		let hac_xs = hac_xs_res.unwrap();
		let hac_ys = hac_ys_res.unwrap();

		let ext_xs = DVector::from_vec(xs.clone());
		let ext_ys = DVector::from_vec(ys.clone());

		// Catch panics
		let ext_res = catch_unwind(|| { let _ = ext_xs.clone().add(ext_ys.clone()); });
		let hac_res = catch_unwind(|| { let _ = add(hac_xs.clone(), hac_ys.clone()).unwrap(); });

		println!("Prepare!");
		println!("ext_res {}", ext_res.is_ok());
		println!("hac_res {}", hac_res.is_ok());

		if ext_res.is_ok() && hac_res.is_ok() {
			println!("Enter!");
			let hac_op = add(hac_xs, hac_ys).unwrap();
			let ext_op = ext_xs + ext_ys;
	
			TestResult::from_bool(assert_thing(hac_op, ext_op))
		} else if ext_res.is_err() && hac_res.is_err() {
			TestResult::from_bool(true)
		} else {
			TestResult::from_bool(false)
		}
	}
	QuickCheck::new()
		.tests(100)
		.min_tests_passed(100)
		.max_tests(1000000)
		.quickcheck(helper as fn(Vec<i128>, Vec<i128>) -> TestResult);
}

//#[test]
fn test_sub_vectors() {
	fn helper(xs: Vec<i128>, ys: Vec<i128>) -> TestResult {
		if xs.len() != ys.len() {
			return TestResult::discard()
		}
		if xs.len() == 0 && ys.len() == 0 {
			return TestResult::discard()
		}

		let hac_xs_res = new(xs.len(), 1, Seq::<Scalar>::from_vec(xs.clone()));
		let hac_ys_res = new(ys.len(), 1, Seq::<Scalar>::from_vec(ys.clone()));
		if hac_xs_res.is_err() {
			return TestResult::from_bool(false)
		}
		if hac_ys_res.is_err() {
			return TestResult::from_bool(false)
		}

		let hac_xs = hac_xs_res.unwrap();
		let hac_ys = hac_ys_res.unwrap();

		let ext_xs = DVector::from_vec(xs.clone());
		let ext_ys = DVector::from_vec(ys.clone());

		// Catch panics
		let ext_res = catch_unwind(|| { let _ = ext_xs.clone().sub(ext_ys.clone()); });
		let hac_res = catch_unwind(|| { let _ = sub(hac_xs.clone(), hac_ys.clone()).unwrap(); });

		println!("Prepare!");
		println!("ext_res {}", ext_res.is_ok());
		println!("hac_res {}", hac_res.is_ok());

		if ext_res.is_ok() && hac_res.is_ok() {
			println!("Enter!");
			let hac_op = sub(hac_xs, hac_ys).unwrap();
			let ext_op = ext_xs - ext_ys;
	
			TestResult::from_bool(assert_thing(hac_op, ext_op))
		} else if ext_res.is_err() && hac_res.is_err() {
			TestResult::from_bool(true)
		} else {
			TestResult::from_bool(false)
		}
	}
	QuickCheck::new()
		.tests(100)
		.min_tests_passed(100)
		.max_tests(1000000)
		.quickcheck(helper as fn(Vec<i128>, Vec<i128>) -> TestResult);
}

//#[test]
fn test_hadamard_vectors() {
	fn helper(xs: Vec<i128>, ys: Vec<i128>) -> TestResult {
		if xs.len() != ys.len() {
			return TestResult::discard()
		}
		if xs.len() == 0 && ys.len() == 0 {
			return TestResult::discard()
		}

		let hac_xs_res = new(xs.len(), 1, Seq::<Scalar>::from_vec(xs.clone()));
		let hac_ys_res = new(ys.len(), 1, Seq::<Scalar>::from_vec(ys.clone()));
		if hac_xs_res.is_err() {
			return TestResult::from_bool(false)
		}
		if hac_ys_res.is_err() {
			return TestResult::from_bool(false)
		}

		let hac_xs = hac_xs_res.unwrap();
		let hac_ys = hac_ys_res.unwrap();

		let ext_xs = DVector::from_vec(xs.clone());
		let ext_ys = DVector::from_vec(ys.clone());

		// Catch panics
		let ext_res = catch_unwind(|| { let _ = ext_xs.clone().mul(ext_ys.clone()); });
		let hac_res = catch_unwind(|| { let _ = hadamard(hac_xs.clone(), hac_ys.clone()).unwrap(); });

		println!("Prepare!");
		println!("ext_res {}", ext_res.is_ok());
		println!("hac_res {}", hac_res.is_ok());

		if ext_res.is_ok() && hac_res.is_ok() {
			println!("Enter!");
			let hac_op = hadamard(hac_xs, hac_ys).unwrap();
			let ext_op = ext_xs * ext_ys;
	
			TestResult::from_bool(assert_thing(hac_op, ext_op))
		} else if ext_res.is_err() && hac_res.is_err() {
			TestResult::from_bool(true)
		} else {
			TestResult::from_bool(false)
		}
	}
	QuickCheck::new()
		.tests(100)
		.min_tests_passed(100)
		.max_tests(1000000)
		.quickcheck(helper as fn(Vec<i128>, Vec<i128>) -> TestResult);
}

#[test]
fn test_transpose() {
	fn helper(xs: Vec<i128>) -> TestResult {
		if xs.len() == 0 {
			return TestResult::discard()
		}

		let hac_res = new(xs.len(), 1, Seq::<Scalar>::from_vec(xs.clone()));
		if hac_res.is_err() {
			return TestResult::from_bool(false)
		}

		let hac = hac_res.unwrap();
		let ext = DMatrix::from_vec(xs.len(), 1, xs.clone());

		// Catch panics
		let ext_res = catch_unwind(|| { let _ = ext.transpose(); });
		let hac_res = catch_unwind(|| { let _ = hacspec_linalg::transpose(hac.clone()); });

		println!("Prepare!");
		println!("ext_res {}", ext_res.is_ok());
		println!("hac_res {}", hac_res.is_ok());

		if ext_res.is_ok() && hac_res.is_ok() {
			println!("Enter!");
			let hac_op = transpose(hac);
			let ext_op = ext.transpose();
	
			println!("Dim1: ({}, {}), Dim2: ({}, {})", hac_op.0.0, hac_op.0.1, ext_op.ncols(), ext_op.ncols());
			TestResult::from_bool(hac_op.0 == (ext_op.nrows(), ext_op.ncols()))
		} else if ext_res.is_err() && hac_res.is_err() {
			TestResult::from_bool(true)
		} else {
			TestResult::from_bool(false)
		}
	}
	QuickCheck::new()
		.tests(100)
		.min_tests_passed(100)
		.max_tests(1000000)
		.quickcheck(helper as fn(Vec<i128>) -> TestResult);
}

//#[test]
fn test_mul_matrices() {
	fn helper(xs: Vec<i128>, ys: Vec<i128>, m: usize, n: usize, p: usize) -> TestResult {
		if xs.len() != m*n {
			return TestResult::discard()
		}
		if ys.len() != n*p {
			return TestResult::discard()
		}
		if xs.len() == 0 && ys.len() == 0 {
			return TestResult::discard()
		}

		let hac_xs_res = new(m, n, Seq::<Scalar>::from_vec(xs.clone()));
		let hac_ys_res = new(n, p, Seq::<Scalar>::from_vec(ys.clone()));
		if hac_xs_res.is_err() {
			return TestResult::from_bool(false)
		}
		if hac_ys_res.is_err() {
			return TestResult::from_bool(false)
		}

		let hac_xs = hac_xs_res.unwrap();
		let hac_ys = hac_ys_res.unwrap();

		let ext_xs = DMatrix::from_vec(m, n, xs.clone());
		let ext_ys = DMatrix::from_vec(n, p, ys.clone());

		// Catch panics
		let ext_res = catch_unwind(|| { let _ = ext_xs.clone().mul(ext_ys.clone()); });
		let hac_res = catch_unwind(|| { let _ = mul(hac_xs.clone(), hac_ys.clone()).unwrap(); });

		println!("Prepare!");
		println!("ext_res {}", ext_res.is_ok());
		println!("hac_res {}", hac_res.is_ok());

		if ext_res.is_ok() && hac_res.is_ok() {
			println!("Enter!");
			let hac_op = mul(hac_xs, hac_ys).unwrap();
			let ext_op = ext_xs * ext_ys;
	
			TestResult::from_bool(assert_thing2(hac_op, ext_op))
		} else if ext_res.is_err() && hac_res.is_err() {
			TestResult::from_bool(true)
		} else {
			TestResult::from_bool(false)
		}
	}
	QuickCheck::new()
		.tests(1000)
		.min_tests_passed(1000)
		.max_tests(100000000000)
		.quickcheck(helper as fn(Vec<i128>, Vec<i128>, usize, usize, usize) -> TestResult);
}
