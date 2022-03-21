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

#[test]
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

#[test]
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

#[test]
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

#[test]
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
