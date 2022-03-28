use hacspec_lib::prelude::*;
use hacspec_linalg::*; 

// === Helper functions ===

// Assert that two hacspec matrices are equal
fn assert_hacs(x: Matrix, y: Matrix) -> bool {
	if x.0 != y.0 {
		false
	} else {
		let zipped = x.1.iter().zip(y.1.iter());

		for z in zipped {
			if z.0 != z.1 {
				panic!("{:?} == {:?}, ({},{}) == ({},{})", x.1.native_slice(), y.1.native_slice(), x.0.0, x.0.1, y.0.0, y.0.1)
			}
		}

		true
	}
}

// Turn vector of i128's to vector of BigInts
fn bv(xs:Vec<i128>) -> Vec<BigInt> {
	xs.into_iter().map(|x| BigInt::from(x)).collect()
}

// === Tests ===

#[test]
fn test_unit_zeros() {
	let rs = bv(vec![0,0, 0,0, 0,0, 0,0, 0,0]);

	let hac_op  = zeros(2, 5).unwrap();
	let hac_rs = new(2, 5, Seq::<Scalar>::from_vec(rs.clone())).unwrap();

	assert!(assert_hacs(hac_op, hac_rs));
}

#[test]
fn test_unit_identity() {
	let rs = bv(vec![1,0,0, 0,1,0, 0,0,1]);

	let hac_op  = identity(3).unwrap();
	let hac_rs = new(3,3, Seq::<Scalar>::from_vec(rs.clone())).unwrap();

	assert!(assert_hacs(hac_op, hac_rs));
}

#[test]
fn test_unit_index() {
	let xs = bv(vec![0,1,2, 3,4,5]);
	let hac_xs = new(2,3, Seq::<Scalar>::from_vec(xs.clone())).unwrap();

	assert!(index(0,0,hac_xs.clone()).unwrap() == 0.into());
	assert!(index(0,1,hac_xs.clone()).unwrap() == 1.into());
	assert!(index(0,2,hac_xs.clone()).unwrap() == 2.into());
	assert!(index(1,0,hac_xs.clone()).unwrap() == 3.into());
	assert!(index(1,1,hac_xs.clone()).unwrap() == 4.into());
	assert!(index(1,2,hac_xs.clone()).unwrap() == 5.into());
}

#[test]
fn test_unit_transpose() {
	let xs = bv(vec![
		0,1,2,
		3,4,5]);
	let rs = bv(vec![
		0,3,
		1,4,
		2,5]);

	let hac_xs = new(2,3, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
	let hac_rs = new(3,2, Seq::<Scalar>::from_vec(rs.clone())).unwrap();

	let hac_op = transpose(hac_xs.clone());
	assert_hacs(hac_op, hac_rs);
}

#[test]
fn test_unit_slice() {
	let xs = bv(vec![
		0,1,2, 3,
		4,5,6, 7,
		8,9,10,11]);
	let rs = bv(vec![
		5,6, 7,
		9,10,11]);

	let hac_xs  = new(3, 4, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
	let hac_rs = new(2, 3, Seq::<Scalar>::from_vec(rs.clone())).unwrap();

	let hac_op = slice((1,1), 2, 3, hac_xs.clone()).unwrap();

	assert!(assert_hacs(hac_op, hac_rs));
}

#[test]
fn test_unit_add() {
	let xs = bv(vec![0,1,2, 3,4,5]);
	let ys = bv(vec![7,3,6, 2,4,3]);
	let rs = bv(vec![7,4,8, 5,8,8]);

	let hac_xs = new(2,3, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
	let hac_ys = new(2,3, Seq::<Scalar>::from_vec(ys.clone())).unwrap();
	let hac_rs = new(2,3, Seq::<Scalar>::from_vec(rs.clone())).unwrap();

	let hac_op = add(hac_xs.clone(), hac_ys.clone()).unwrap();
	assert_hacs(hac_op, hac_rs);
}

#[test]
fn test_unit_sub() {
	let xs = bv(vec![ 0, 1, 2, 3,4,5]);
	let ys = bv(vec![ 7, 3, 6, 2,4,3]);
	let rs = bv(vec![-7,-2,-4, 1,0,2]);

	let hac_xs = new(2,3, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
	let hac_ys = new(2,3, Seq::<Scalar>::from_vec(ys.clone())).unwrap();
	let hac_rs = new(2,3, Seq::<Scalar>::from_vec(rs.clone())).unwrap();

	let hac_op = sub(hac_xs.clone(), hac_ys.clone()).unwrap();
	assert_hacs(hac_op, hac_rs);
}

#[test]
fn test_unit_component_mul() {
	let xs = bv(vec![0,1, 2, 2, 4, 5]);
	let ys = bv(vec![7,3, 6, 3, 4, 3]);
	let rs = bv(vec![0,3,12, 6,16,15]);

	let hac_xs = new(2,3, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
	let hac_ys = new(2,3, Seq::<Scalar>::from_vec(ys.clone())).unwrap();
	let hac_rs = new(2,3, Seq::<Scalar>::from_vec(rs.clone())).unwrap();

	let hac_op = component_mul(hac_xs.clone(), hac_ys.clone()).unwrap();
	assert_hacs(hac_op, hac_rs);
}

#[test]
fn test_unit_mul() {
	let xs = bv(vec![1,2,1, 0,1,1]);
	let ys = bv(vec![2,5,1,1, 6,7,1,1, 1,1,1,1]);
	let rs = bv(vec![15,20,4,4, 7,8,2,2]);

	let hac_xs = new(2, 3, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
	let hac_ys = new(3, 4, Seq::<Scalar>::from_vec(ys.clone())).unwrap();
	let hac_rs = new(2, 4, Seq::<Scalar>::from_vec(rs.clone())).unwrap();

	let hac_mul = mul(hac_xs, hac_ys).unwrap();

	assert!(assert_hacs(hac_mul, hac_rs));
}
