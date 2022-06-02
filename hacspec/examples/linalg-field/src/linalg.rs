use hacspec_lib::*;

pub type DimType = usize;
pub type MatrixEntry = hacspec_ristretto::Scalar;
pub type Dims = (DimType, DimType);
pub type Matrix = (Dims, Seq<MatrixEntry>);

pub fn matrix_new(rows: DimType, cols: DimType, seq: Seq<MatrixEntry>) -> Result<Matrix, ()> {
    if seq.len() > 0 && rows * cols == seq.len() {
        Result::<Matrix, ()>::Ok(((rows, cols), seq))
    } else {
        Result::<Matrix, ()>::Err(())
    }
}

pub fn zeros(n: DimType, m: DimType) -> Result<Matrix, ()> {
    matrix_new(n, m, Seq::<MatrixEntry>::new(n * m))
}

pub fn ones(n: DimType, m: DimType) -> Result<Matrix, ()> {
    let mut ret = Seq::<MatrixEntry>::new(n * m);

    for i in 0..n * m {
        ret[i] = MatrixEntry::ONE();
    }

    matrix_new(n, m, ret)
}

pub fn identity(n: DimType) -> Result<Matrix, ()> {
    let mut ret = Seq::<MatrixEntry>::new(n * n);

    for i in 0..n {
        let index = i * n + i;
        ret[index] = MatrixEntry::ONE();
    }

    matrix_new(n, n, ret)
}

pub fn index(m: Matrix, i: DimType, j: DimType) -> Result<MatrixEntry, ()> {
    let (dim, seq) = m;
    let (rows, cols) = dim;
    let index = i * cols + j;

    if index >= rows * cols {
        Result::<MatrixEntry, ()>::Err(())
    } else {
        Result::<MatrixEntry, ()>::Ok(seq[index])
    }
}

pub fn transpose(matrix: Matrix) -> Matrix {
    let (dim, seq) = matrix;
    let (rows, cols) = dim;
    let mut ret = Seq::<MatrixEntry>::new(seq.len());

    for i in 0..rows {
        for j in 0..cols {
            let seq_index = j * rows + i;
            let index = i * cols + j;
            ret[seq_index] = seq[index]
        }
    }

    ((cols, rows), ret)
}

pub fn slice(matrix: Matrix, start: Dims, len: Dims) -> Result<Matrix, ()> {
    let (dim, seq) = matrix;
    let (rows, cols) = dim;
    let (start_row, start_col) = start;
    let (len_rows, len_cols) = len;
    let start_index = start_row * cols + start_col;
    let mut ret = Seq::<MatrixEntry>::new(len_rows * len_cols);
    let mut res = Result::<Matrix, ()>::Err(());

    if start_index + len_rows * len_cols <= rows * cols {
        for i in 0..len_rows {
            for j in 0..len_cols {
                let ret_index = i * len_cols + j;
                let seq_index = (start_row + i) * cols + (start_col + j);
                ret[ret_index] = seq[seq_index].clone()
            }
        }

        res = matrix_new(len_rows, len_cols, ret);
    }

    res
}

pub fn matrix_scale(matrix: Matrix, scalar: MatrixEntry) -> Matrix {
    let (dim, seq) = matrix;
    let mut ret = Seq::<MatrixEntry>::new(seq.len());

    for i in 0..seq.len() {
        ret[i] = scalar * seq[i].clone()
    }

    (dim, ret)
}

pub fn matrix_add(matrix_1: Matrix, matrix_2: Matrix) -> Result<Matrix, ()> {
    let (m1_dim, m1_s) = matrix_1;
    let (m2_dim, m2_s) = matrix_2;
    let mut ret = Seq::<MatrixEntry>::new(m1_s.len());
    let mut res = Result::<Matrix, ()>::Err(());

    if m1_dim == m2_dim {
        for i in 0..m1_s.len() {
            ret[i] = m1_s[i] + m2_s[i]
        }
        res = Result::<Matrix, ()>::Ok((m1_dim, ret))
    }
    res
}

pub fn matrix_sub(matrix_1: Matrix, matrix_2: Matrix) -> Result<Matrix, ()> {
    let (m1_dim, m1_s) = matrix_1;
    let (m2_dim, m2_s) = matrix_2;
    let mut ret = Seq::<MatrixEntry>::new(m1_s.len());
    let mut res = Result::<Matrix, ()>::Err(());

    if m1_dim == m2_dim {
        for i in 0..m1_s.len() {
            ret[i] = m1_s[i].clone() - m2_s[i].clone()
        }
        res = Result::<Matrix, ()>::Ok((m1_dim, ret))
    }
    res
}

pub fn matrix_component_mul(matrix_1: Matrix, matrix_2: Matrix) -> Result<Matrix, ()> {
    let (m1_dim, m1_s) = matrix_1;
    let (m2_dim, m2_s) = matrix_2;
    let mut ret = Seq::<MatrixEntry>::new(m1_s.len());
    let mut res = Result::<Matrix, ()>::Err(());

    if m1_dim == m2_dim {
        for i in 0..m1_s.len() {
            ret[i] = m1_s[i].clone() * m2_s[i].clone()
        }
        res = Result::<Matrix, ()>::Ok((m1_dim, ret))
    }
    res
}

pub fn matrix_mul(matrix_1: Matrix, matrix_2: Matrix) -> Result<Matrix, ()> {
    let (dim_1, seq_1) = matrix_1;
    let (dim_2, seq_2) = matrix_2;
    let (m, n) = dim_1;
    let (n_2, p) = dim_2;
    let mut ret = Seq::<MatrixEntry>::new(m * p);
    let mut res = Result::<Matrix, ()>::Err(());

    if n == n_2 {
        for i in 0..m {
            for j in 0..p {
                let mut acc = MatrixEntry::ZERO();
                let index = i * p + j;

                for k in 0..n {
                    let index_1 = i * n + k;
                    let index_2 = k * p + j;

                    acc = acc + seq_1[index_1].clone() * seq_2[index_2].clone();
                }

                ret[index] = acc
            }
        }

        res = matrix_new(m, p, ret)
    }

    res
}
