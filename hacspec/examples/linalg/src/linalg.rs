use hacspec_lib::*;

pub type DimType = usize;
pub type Scalar  = i128;
pub type Dims    = (DimType, DimType);
pub type Matrix  = (Dims, Seq<Scalar>);

pub fn new(rows: DimType, cols: DimType, vec: Seq<Scalar>) -> Result<Matrix, ()>{
	if rows <= 0 || cols <= 0 {
		Result::<Matrix, ()>::Err(())
	} else {
		Result::<Matrix, ()>::Ok(((rows,cols), vec))
	}
}

pub fn index(i: DimType, j: DimType, matrix: Matrix) -> Result<Scalar, ()> {
	let (dim, seq) = matrix;
	let (rows, cols) = dim;
	let index = i*cols + j;

	if index >= rows*cols {
		Result::<Scalar, ()>::Err(())
	} else {
		Result::<Scalar, ()>::Ok(seq[index])
	}
}

pub fn transpose(matrix: Matrix) -> Matrix {
	let (dim, seq) = matrix;
	let (rows, cols) = dim;
	let mut ret = Seq::<Scalar>::new(seq.len());

	for i in 0..rows {
		for j in 0..cols {
			let seq_index = i * cols + j;
			let index = j * cols + i;
			ret[seq_index] = seq[index].clone()
		}
	}

	new(cols, rows, ret).unwrap()
}

pub fn add(m1: Matrix, m2: Matrix) -> Result<Matrix, ()> {
	let (m1_dim, m1_s) = m1;
	let (m2_dim, m2_s) = m2;
	let mut ret = Seq::<Scalar>::new(m1_s.len());
	let mut res = Result::<Matrix, ()>::Err(());

	if m1_dim == m2_dim {
		for i in 0..m1_s.len() {
			ret[i] = m1_s[i] + m2_s[i]
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
			ret[i] = m1_s[i] - m2_s[i]
		}
		res = Result::<Matrix, ()>::Ok((m1_dim,ret))
	}
	res
}

pub fn hadamard(m1: Matrix, m2: Matrix) -> Result<Matrix, ()> {
	let (m1_dim, m1_s) = m1;
	let (m2_dim, m2_s) = m2;
	let mut ret = Seq::<Scalar>::new(m1_s.len());
	let mut res = Result::<Matrix, ()>::Err(());

	if m1_dim == m2_dim {
		for i in 0..m1_s.len() {
			ret[i] = m1_s[i] * m2_s[i]
		}
		res = Result::<Matrix, ()>::Ok((m1_dim,ret))
	}
	res
}

pub fn dot(m1: Matrix, m2: Matrix) {
}
