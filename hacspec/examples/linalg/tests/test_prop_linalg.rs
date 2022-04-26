extern crate nalgebra;
extern crate quickcheck;

use hacspec_lib::prelude::*;
use hacspec_linalg::*;

use nalgebra::DMatrix;
use quickcheck::*;

type IntSize = i32;

// === Helper functions ===

fn assert_matrices(x: Matrix, y: DMatrix<Scalar>) -> bool {
    if x.0 == (y.nrows(), y.ncols()) {
        let y = y.transpose();
        let zipped = x.1.iter().zip(y.iter());

        for z in zipped {
            if z.0 != z.1 {
                break;
            }
        }

        return true;
    }
    panic!(
        "({:?}) == ({:?}), ({},{}) == ({},{})",
        x.1.native_slice(),
        y.as_slice(),
        x.0 .0,
        x.0 .1,
        y.nrows(),
        y.ncols()
    );
}

fn quickcheck(helper: impl Testable) {
    QuickCheck::new()
        .tests(100)
        .min_tests_passed(100)
        .max_tests(1000000)
        .quickcheck(helper);
}

fn castVec(xs: Vec<IntSize>) -> Vec<Scalar> {
    xs.into_iter().map(|x| x.into()).collect()
}

fn dmatrix(n: usize, m: usize, xs: Vec<Scalar>) -> DMatrix<Scalar> {
    DMatrix::from_vec(m, n, xs).transpose()
}

// === Test Functions ===

#[test]
fn test_prop_zeros() {
    fn helper(n: u8, m: u8) -> TestResult {
        let n = n as usize;
        let m = m as usize;

        if n * m == 0 {
            return TestResult::discard();
        }

        let hac = zeros(n, m).unwrap();
        let ext = DMatrix::zeros(n, m);

        TestResult::from_bool(assert_matrices(hac, ext))
    }
    quickcheck(helper as fn(u8, u8) -> TestResult);
}

#[test]
fn test_prop_ones() {
    fn helper(n: u8, m: u8) -> TestResult {
        let n = n as usize;
        let m = m as usize;

        if n * m == 0 {
            return TestResult::discard();
        }

        let hac = ones(n, m).unwrap();
        let mut ext = DMatrix::zeros(n, m);
        ext.fill(Scalar::ONE());

        TestResult::from_bool(assert_matrices(hac, ext.clone()))
    }
    quickcheck(helper as fn(u8, u8) -> TestResult);
}

#[test]
fn test_prop_identity() {
    fn helper(n: u8) -> TestResult {
        let n = n as usize;

        if n == 0 {
            return TestResult::discard();
        }

        let hac = identity(n).unwrap();
        let ext = DMatrix::identity(n, n);

        TestResult::from_bool(assert_matrices(hac, ext))
    }
    quickcheck(helper as fn(u8) -> TestResult);
}

#[test]
fn test_prop_index() {
    fn helper(xs: Vec<IntSize>, n: u8, m: u8) -> TestResult {
        let mut xs = castVec(xs);
        let n = n as usize;
        let m = m as usize;

        if n * m == 0 || n * m > xs.len() {
            return TestResult::discard();
        }

        xs.truncate(n * m);

        let hac = new(n, m, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
        let ext = dmatrix(n, m, xs.clone());

        let mut eq = true;
        for i in 0..n {
            for j in 0..m {
                let hac_op = index(hac.clone(), i, j).unwrap();
                let ext_op = ext.index((i, j));

                if hac_op != *ext_op {
                    eq = false
                }
            }
        }

        TestResult::from_bool(eq)
    }
    quickcheck(helper as fn(Vec<IntSize>, u8, u8) -> TestResult);
}

#[test]
fn test_prop_transpose() {
    fn helper(xs: Vec<IntSize>, n: u8, m: u8) -> TestResult {
        let mut xs = castVec(xs);
        let n = n as usize;
        let m = m as usize;

        if n * m == 0 || n * m > xs.len() {
            return TestResult::discard();
        }

        xs.truncate(n * m);

        let hac = new(n, m, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
        let ext = dmatrix(n, m, xs.clone());

        let hac_op = transpose(hac);
        let ext_op = ext.transpose();

        TestResult::from_bool(assert_matrices(hac_op, ext_op))
    }
    quickcheck(helper as fn(Vec<IntSize>, u8, u8) -> TestResult);
}

#[test]
fn test_prop_slice() {
    fn helper(xs: Vec<IntSize>, n: u8, m: u8) -> TestResult {
        let mut xs = castVec(xs);
        let n = n as usize;
        let m = m as usize;

        if n * m == 0 || n * m > xs.len() {
            return TestResult::discard();
        }

        xs.truncate(n * m);

        let hac = new(n, m, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
        let ext = dmatrix(n, m, xs.clone());

        // Try all combinations of slices
        let mut eq = true;
        for i in 0..n - 1 {
            for j in 0..m - 1 {
                for k in 1..n - i - 1 {
                    for l in 1..m - j - 1 {
                        let hac_op = slice(hac.clone(), (i, j), (k, l)).unwrap();
                        let ext_op: DMatrix<Scalar> = ext.slice((i, j), (k, l)).into();

                        if !assert_matrices(hac_op.clone(), ext_op.clone()) {
                            eq = false
                        }
                    }
                }
            }
        }

        TestResult::from_bool(eq)
    }
    quickcheck(helper as fn(Vec<IntSize>, u8, u8) -> TestResult);
}

#[test]
fn test_prop_scale() {
    fn helper(xs: Vec<IntSize>, n: u8, m: u8, scalar: IntSize) -> TestResult {
        let mut xs = castVec(xs);
        let n = n as usize;
        let m = m as usize;
        let scalar = scalar as Scalar;

        if n * m == 0 || n * m > xs.len() {
            return TestResult::discard();
        }

        xs.truncate(n * m);

        let hac = new(n, m, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
        let ext = dmatrix(n, m, xs.clone());

        let hac_op = scale(hac, scalar);
        let ext_op = ext * scalar;

        TestResult::from_bool(assert_matrices(hac_op, ext_op))
    }
    quickcheck(helper as fn(Vec<IntSize>, u8, u8, IntSize) -> TestResult);
}

#[test]
fn test_prop_add() {
    fn helper(xs: Vec<IntSize>, ys: Vec<IntSize>, n: u8, m: u8) -> TestResult {
        let mut xs = castVec(xs);
        let mut ys = castVec(ys);
        let n = n as usize;
        let m = m as usize;

        if n * m == 0 || n * m > xs.len() || n * m > ys.len() {
            return TestResult::discard();
        }

        xs.truncate(n * m);
        ys.truncate(n * m);

        let hac_xs = new(n, m, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
        let hac_ys = new(n, m, Seq::<Scalar>::from_vec(ys.clone())).unwrap();

        let ext_xs = dmatrix(n, m, xs.clone());
        let ext_ys = dmatrix(n, m, ys.clone());

        let hac_op = add(hac_xs, hac_ys).unwrap();
        let ext_op = ext_xs + ext_ys;

        TestResult::from_bool(assert_matrices(hac_op, ext_op))
    }
    quickcheck(helper as fn(Vec<IntSize>, Vec<IntSize>, u8, u8) -> TestResult);
}

#[test]
fn test_prop_sub() {
    fn helper(xs: Vec<IntSize>, ys: Vec<IntSize>, n: u8, m: u8) -> TestResult {
        let mut xs = castVec(xs);
        let mut ys = castVec(ys);
        let n = n as usize;
        let m = m as usize;

        if n * m == 0 || n * m > xs.len() || n * m > ys.len() {
            return TestResult::discard();
        }

        xs.truncate(n * m);
        ys.truncate(n * m);

        let hac_xs = new(n, m, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
        let hac_ys = new(n, m, Seq::<Scalar>::from_vec(ys.clone())).unwrap();

        let ext_xs = dmatrix(n, m, xs.clone());
        let ext_ys = dmatrix(n, m, ys.clone());

        let hac_op = sub(hac_xs, hac_ys).unwrap();
        let ext_op = ext_xs - ext_ys;

        TestResult::from_bool(assert_matrices(hac_op, ext_op))
    }
    quickcheck(helper as fn(Vec<IntSize>, Vec<IntSize>, u8, u8) -> TestResult);
}

#[test]
fn test_prop_component_mul() {
    fn helper(xs: Vec<IntSize>, ys: Vec<IntSize>, n: u8, m: u8) -> TestResult {
        let mut xs = castVec(xs);
        let mut ys = castVec(ys);
        let n = n as usize;
        let m = m as usize;

        if n * m == 0 || n * m > xs.len() || n * m > ys.len() {
            return TestResult::discard();
        }

        xs.truncate(n * m);
        ys.truncate(n * m);

        let hac_xs = new(n, m, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
        let hac_ys = new(n, m, Seq::<Scalar>::from_vec(ys.clone())).unwrap();

        let ext_xs = dmatrix(n, m, xs.clone());
        let ext_ys = dmatrix(n, m, ys.clone());

        let hac_op = component_mul(hac_xs, hac_ys).unwrap();
        let ext_op = ext_xs.component_mul(&ext_ys);

        TestResult::from_bool(assert_matrices(hac_op, ext_op))
    }
    quickcheck(helper as fn(Vec<IntSize>, Vec<IntSize>, u8, u8) -> TestResult);
}

#[test]
fn test_prop_mul() {
    fn helper(xs: Vec<IntSize>, ys: Vec<IntSize>, n: u8, m: u8, p: u8) -> TestResult {
        let mut xs = castVec(xs);
        let mut ys = castVec(ys);
        let n = n as usize;
        let m = m as usize;
        let p = p as usize;

        if n * m * p == 0 || n * m > xs.len() || m * p > ys.len() {
            return TestResult::discard();
        }

        xs.truncate(n * m);
        ys.truncate(m * p);

        let hac_xs = new(n, m, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
        let hac_ys = new(m, p, Seq::<Scalar>::from_vec(ys.clone())).unwrap();

        let ext_xs = dmatrix(n, m, xs.clone());
        let ext_ys = dmatrix(m, p, ys.clone());

        let hac_op = mul(hac_xs, hac_ys).unwrap();
        let ext_op = ext_xs.mul(&ext_ys);

        TestResult::from_bool(assert_matrices(hac_op, ext_op))
    }
    quickcheck(helper as fn(Vec<IntSize>, Vec<IntSize>, u8, u8, u8) -> TestResult);
}

// === Test Properties ===

#[test]
fn test_prop_multiplicative_identity() {
    fn helper(xs: Vec<IntSize>, ys: Vec<IntSize>, n: u8, m: u8, p: u8) -> TestResult {
        let mut xs = castVec(xs);
        let mut ys = castVec(ys);
        let n = n as usize;
        let m = m as usize;
        let p = p as usize;

        if n * m * p == 0 || n * m > xs.len() || m * p > ys.len() {
            return TestResult::discard();
        }

        xs.truncate(n * m);
        ys.truncate(m * p);

        let hac_xs = new(n, m, Seq::<Scalar>::from_vec(xs.clone())).unwrap();
        let hac_ys = new(m, p, Seq::<Scalar>::from_vec(ys.clone())).unwrap();

        let ext_xs = dmatrix(n, m, xs.clone());
        let ext_ys = dmatrix(m, p, ys.clone());

        let hac_op = mul(hac_xs, hac_ys).unwrap();
        let ext_op = ext_xs.mul(&ext_ys);

        TestResult::from_bool(assert_matrices(hac_op, ext_op))
    }
    quickcheck(helper as fn(Vec<IntSize>, Vec<IntSize>, u8, u8, u8) -> TestResult);
}
