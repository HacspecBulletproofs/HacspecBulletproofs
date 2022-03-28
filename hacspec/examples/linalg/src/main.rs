use hacspec_lib::prelude::*;
use hacspec_linalg::*; 

fn bv(xs: Vec<i128>) -> Vec<BigInt> {
	fn conv(x: &i128) -> BigInt {
		BigInt::from(*x)
	}
	xs.iter().map(conv).collect()
}

pub fn clise(start: (DimType, DimType), new_rows: DimType, new_cols: DimType, matrix: Matrix) -> Result<Matrix, ()> {
	let (dim, seq) = matrix;
	let (rows, cols) = dim;
	let (start_row, start_col) = start;
	let start_index = start_row*cols + start_col;
	let mut ret = Seq::<Scalar>::new(new_rows*new_cols);
	let mut res = Result::<Matrix, ()>::Err(());

	println!("start_loop");
	if start_index + new_rows*new_cols <= rows*cols {
		for i in 0..new_rows {
			for j in 0..new_cols {
				let ret_index = i*new_cols + j;
				let seq_index = (start_row+i)*cols + (start_col+j);
				println!("({},{}): {}", i,j,ret_index);
				ret[ret_index] = seq[seq_index].clone()
			}
		}
		println!("done");
		println!("({},{}): {}",new_rows,new_cols, ret.len());

		res = new(new_rows, new_cols, ret);
	}

	res
}

fn main() {
	let xs = bv(vec![
		0,1,
		2,3]);

	let hac_xs = new(2, 2, Seq::<Scalar>::from_vec(xs.clone())).unwrap();

	let hac_slice = clise((1,1), 1, 1, hac_xs).unwrap();

	println!("{:?}", hac_slice.1);
}

//0, 3, 6
//1, 4, 7
//2, 5, 8
