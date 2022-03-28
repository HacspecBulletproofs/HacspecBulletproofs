use hacspec_lib::*;

pub type DimType = usize;
pub type Scalar  = BigInt;
pub type Dims    = (DimType, DimType);
pub type Matrix  = (Dims, Seq<Scalar>);

pub fn new(rows: DimType, cols: DimType, seq: Seq<Scalar>) -> Result<Matrix, ()>{
	if rows <= 0 || cols <= 0 || rows*cols != seq.len() {
		Result::<Matrix, ()>::Err(())
	} else {
		Result::<Matrix, ()>::Ok(((rows,cols), seq))
	}
}

pub fn zeros(n:DimType, m:DimType) -> Result::<Matrix,()> {
	new(n,m,Seq::<Scalar>::new(n*m))
}

pub fn identity(n: DimType) -> Result::<Matrix,()> {
	let mut ret = Seq::<Scalar>::new(n*n);

	for i in 0..n {
		let index = i * n + i;
		ret[index] = Scalar::ONE()
	}

	new(n,n,ret)
}

pub fn index(i: DimType, j: DimType, m: Matrix) -> Result<Scalar, ()> {
	let (dim, seq) = m;
	let (rows, cols) = dim;
	let index = i*cols + j;

	if index >= rows*cols {
		Result::<Scalar, ()>::Err(())
	} else {
		Result::<Scalar, ()>::Ok(seq[index].clone())
	}
}

pub fn transpose(matrix: Matrix) -> Matrix {
	let (dim, seq) = matrix;
	let (rows, cols) = dim;
	let mut ret = Seq::<Scalar>::new(seq.len());

	for i in 0..rows {
		for j in 0..cols {
			let seq_index = j * rows + i;
			let index = i * cols + j;
			ret[seq_index] = seq[index].clone()
		}
	}

	new(cols, rows, ret).unwrap()
}

pub fn slice(start: (DimType, DimType), new_rows: DimType, new_cols: DimType, matrix: Matrix) -> Result<Matrix, ()> {
	let (dim, seq) = matrix;
	let (rows, cols) = dim;
	let (start_row, start_col) = start;
	let start_index = start_row*cols + start_col;
	let mut ret = Seq::<Scalar>::new(new_rows*new_cols);
	let mut res = Result::<Matrix, ()>::Err(());

	if start_index + new_rows*new_cols <= rows*cols {
		for i in 0..new_rows {
			for j in 0..new_cols {
				let ret_index = i*new_cols + j;
				let seq_index = (start_row+i)*cols + (start_col+j);
				ret[ret_index] = seq[seq_index].clone()
			}
		}

		res = new(new_rows, new_cols, ret);
	}

	res

}

pub fn add(m1: Matrix, m2: Matrix) -> Result<Matrix, ()> {
	let (m1_dim, m1_s) = m1;
	let (m2_dim, m2_s) = m2;
	let mut ret = Seq::<Scalar>::new(m1_s.len());
	let mut res = Result::<Matrix, ()>::Err(());

	if m1_dim == m2_dim {
		for i in 0..m1_s.len() {
			ret[i] = m1_s[i].clone() + m2_s[i].clone()
		}
		res = Result::<Matrix, ()>::Ok((m1_dim,ret))
	}
	res
}

pub fn sub(m1: Matrix, m2: Matrix) -> Result<Matrix, ()> {
	let (m1_dim, m1_s) = m1;
	let (m2_dim, m2_s) = m2;
	let mut ret = Seq::<Scalar>::new(m1_s.len());
	let mut res = Result::<Matrix, ()>::Err(());

	if m1_dim == m2_dim {
		for i in 0..m1_s.len() {
			ret[i] = m1_s[i].clone() - m2_s[i].clone()
		}
		res = Result::<Matrix, ()>::Ok((m1_dim,ret))
	}
	res
}

pub fn component_mul(m1: Matrix, m2: Matrix) -> Result<Matrix, ()> {
	let (m1_dim, m1_s) = m1;
	let (m2_dim, m2_s) = m2;
	let mut ret = Seq::<Scalar>::new(m1_s.len());
	let mut res = Result::<Matrix, ()>::Err(());

	if m1_dim == m2_dim {
		for i in 0..m1_s.len() {
			ret[i] = m1_s[i].clone() * m2_s[i].clone()
		}
		res = Result::<Matrix, ()>::Ok((m1_dim,ret))
	}
	res
}

pub fn mul(m1: Matrix, m2: Matrix) -> Result<Matrix, ()> {
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

					acc = acc + seq_1[index_1].clone() * seq_2[index_2].clone();
				}

				ret[index] = acc
			}
		}

		res = new(m, p, ret)
	}

	res
}
