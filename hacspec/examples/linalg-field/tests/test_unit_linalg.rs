use hacspec_lib::prelude::*;
use hacspec_linalg_field::*;
use std::convert::TryInto;

type IntSize = i32;

// === Helper functions ===

// Assert that two hacspec matrices are equal
fn assert_hacs(x: Matrix, y: Matrix) -> bool {
    if x.0 != y.0 {
        false
    } else {
        let zipped = x.1.iter().zip(y.1.iter());

        for z in zipped {
            if z.0 != z.1 {
                panic!(
                    "{:?} == {:?}, ({},{}) == ({},{})",
                    x.1.native_slice(),
                    y.1.native_slice(),
                    x.0 .0,
                    x.0 .1,
                    y.0 .0,
                    y.0 .1
                )
            }
        }

        true
    }
}

fn cast_vec(xs: Vec<IntSize>) -> Vec<MatrixEntry> {
    xs.into_iter()
        .map(|x| MatrixEntry::from_literal(x.try_into().unwrap()))
        .collect()
}

// === Tests ===

#[test]
fn test_unit_zeros() {
    let rs = cast_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    let hac_op = matrix_zeros(2, 5).unwrap();
    let hac_rs = matrix_new(2, 5, Seq::<MatrixEntry>::from_vec(rs.clone())).unwrap();

    assert!(assert_hacs(hac_op, hac_rs));
}

#[test]
fn test_unit_ones() {
    let rs = cast_vec(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);

    let hac_op = matrix_ones(2, 5).unwrap();
    let hac_rs = matrix_new(2, 5, Seq::<MatrixEntry>::from_vec(rs.clone())).unwrap();

    assert!(assert_hacs(hac_op, hac_rs));
}

#[test]
fn test_unit_identity() {
    let rs = cast_vec(vec![1, 0, 0, 0, 1, 0, 0, 0, 1]);

    let hac_op = matrix_identity(3, 3).unwrap();
    let hac_rs = matrix_new(3, 3, Seq::<MatrixEntry>::from_vec(rs.clone())).unwrap();

    assert!(assert_hacs(hac_op, hac_rs));
}

#[test]
fn test_unit_index() {
    let xs = cast_vec(vec![0, 1, 2, 3, 4, 5]);
    let hac_xs = matrix_new(2, 3, Seq::<MatrixEntry>::from_vec(xs.clone())).unwrap();

    assert!(matrix_index(hac_xs.clone(), 0, 0).unwrap() == MatrixEntry::from_literal(0));
    assert!(matrix_index(hac_xs.clone(), 0, 1).unwrap() == MatrixEntry::from_literal(1));
    assert!(matrix_index(hac_xs.clone(), 0, 2).unwrap() == MatrixEntry::from_literal(2));
    assert!(matrix_index(hac_xs.clone(), 1, 0).unwrap() == MatrixEntry::from_literal(3));
    assert!(matrix_index(hac_xs.clone(), 1, 1).unwrap() == MatrixEntry::from_literal(4));
    assert!(matrix_index(hac_xs.clone(), 1, 2).unwrap() == MatrixEntry::from_literal(5));
}

#[test]
fn test_unit_transpose() {
    let xs = cast_vec(vec![0, 1, 2, 3, 4, 5]);
    let rs = cast_vec(vec![0, 3, 1, 4, 2, 5]);

    let hac_xs = matrix_new(2, 3, Seq::<MatrixEntry>::from_vec(xs.clone())).unwrap();
    let hac_rs = matrix_new(3, 2, Seq::<MatrixEntry>::from_vec(rs.clone())).unwrap();

    let hac_op = matrix_transpose(hac_xs.clone());
    assert_hacs(hac_op, hac_rs);
}

#[test]
fn test_unit_slice() {
    let xs = cast_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    let rs = cast_vec(vec![5, 6, 7, 9, 10, 11]);

    let hac_xs = matrix_new(3, 4, Seq::<MatrixEntry>::from_vec(xs.clone())).unwrap();
    let hac_rs = matrix_new(2, 3, Seq::<MatrixEntry>::from_vec(rs.clone())).unwrap();

    let hac_op = matrix_slice(hac_xs.clone(), (1, 1), (2, 3)).unwrap();

    assert!(assert_hacs(hac_op, hac_rs));
}

#[test]
fn test_unit_scale() {
    let x = 2;
    let xs = cast_vec(vec![0, 1, 2, 3, 4, 5]);
    let rs = cast_vec(vec![0, 2, 4, 6, 8, 10]);

    let hac_xs = matrix_new(2, 3, Seq::<MatrixEntry>::from_vec(xs.clone())).unwrap();
    let hac_rs = matrix_new(2, 3, Seq::<MatrixEntry>::from_vec(rs.clone())).unwrap();

    let hac_op = matrix_scale(hac_xs.clone(), MatrixEntry::from_literal(x));
    assert_hacs(hac_op, hac_rs);
}

#[test]
fn test_unit_matrix_add() {
    let xs = cast_vec(vec![0, 1, 2, 3, 4, 5]);
    let ys = cast_vec(vec![7, 3, 6, 2, 4, 3]);
    let rs = cast_vec(vec![7, 4, 8, 5, 8, 8]);

    let hac_xs = matrix_new(2, 3, Seq::<MatrixEntry>::from_vec(xs.clone())).unwrap();
    let hac_ys = matrix_new(2, 3, Seq::<MatrixEntry>::from_vec(ys.clone())).unwrap();
    let hac_rs = matrix_new(2, 3, Seq::<MatrixEntry>::from_vec(rs.clone())).unwrap();

    let hac_op = matrix_add(hac_xs.clone(), hac_ys.clone()).unwrap();
    assert_hacs(hac_op, hac_rs);
}

#[test]
fn test_unit_sub() {
    let xs = cast_vec(vec![7, 3, 6, 3, 4, 5]);
    let ys = cast_vec(vec![0, 1, 2, 2, 4, 3]);
    let rs = cast_vec(vec![7, 2, 4, 1, 0, 2]);

    let hac_xs = matrix_new(2, 3, Seq::<MatrixEntry>::from_vec(xs.clone())).unwrap();
    let hac_ys = matrix_new(2, 3, Seq::<MatrixEntry>::from_vec(ys.clone())).unwrap();
    let hac_rs = matrix_new(2, 3, Seq::<MatrixEntry>::from_vec(rs.clone())).unwrap();

    let hac_op = matrix_sub(hac_xs.clone(), hac_ys.clone()).unwrap();
    assert_hacs(hac_op, hac_rs);
}

#[test]
fn test_unit_component_mul() {
    let xs = cast_vec(vec![0, 1, 2, 2, 4, 5]);
    let ys = cast_vec(vec![7, 3, 6, 3, 4, 3]);
    let rs = cast_vec(vec![0, 3, 12, 6, 16, 15]);

    let hac_xs = matrix_new(2, 3, Seq::<MatrixEntry>::from_vec(xs.clone())).unwrap();
    let hac_ys = matrix_new(2, 3, Seq::<MatrixEntry>::from_vec(ys.clone())).unwrap();
    let hac_rs = matrix_new(2, 3, Seq::<MatrixEntry>::from_vec(rs.clone())).unwrap();

    let hac_op = matrix_component_mul(hac_xs.clone(), hac_ys.clone()).unwrap();
    assert_hacs(hac_op, hac_rs);
}

#[test]
fn test_unit_mul() {
    let xs = cast_vec(vec![1, 2, 1, 0, 1, 1]);
    let ys = cast_vec(vec![2, 5, 1, 1, 6, 7, 1, 1, 1, 1, 1, 1]);
    let rs = cast_vec(vec![15, 20, 4, 4, 7, 8, 2, 2]);

    let hac_xs = matrix_new(2, 3, Seq::<MatrixEntry>::from_vec(xs.clone())).unwrap();
    let hac_ys = matrix_new(3, 4, Seq::<MatrixEntry>::from_vec(ys.clone())).unwrap();
    let hac_rs = matrix_new(2, 4, Seq::<MatrixEntry>::from_vec(rs.clone())).unwrap();

    let hac_mul = matrix_mul(hac_xs, hac_ys).unwrap();

    assert!(assert_hacs(hac_mul, hac_rs));
}
