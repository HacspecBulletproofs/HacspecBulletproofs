use hacspec_lib::prelude::*;
use hacspec_linalg::*;

pub type Scalar = BigInt;

fn bv(xs: Vec<i128>) -> Vec<BigInt> {
    xs.into_iter().map(|x| BigInt::from(x)).collect()
}

fn main() {}
