# Panics
Interesting. Seems toxic idk
```rust
pub fn panic_if_negative(n: isize) -> isize {
    if n < 0isize {
        Result::<(), ()>::Err(()).unwrap();
    }
    n
}
```

# Generics

## Type aliases doesn't work
This is not possible, rust won't allow it.
```rust
pub type DimType = usize;
pub type Scalar = impl Numeric;
pub type Dims = (DimType, DimType);
pub type Matrix = (Dims, Seq<Scalar>);
```
This is being worked on currently, issue:

<https://github.com/rust-lang/rust/issues/63063>

## Using impl's

Abandoning type aliases, we can do the following:
```rust
pub fn new(rows: DimType, cols: DimType, seq: Seq<impl Numeric>) -> Result<(Dims, Seq<impl Numeric>), ()> {
    if rows <= 0 || cols <= 0 || rows * cols != seq.len() {
        Err(())
    } else {
        Ok(((rows, cols), seq))
    }
}
```
This won't work in hacspec though since `Err()` and `Ok()` must be type annotated:
```rust
pub fn new(rows: DimType, cols: DimType, seq: Seq<impl Numeric>) -> Result<(Dims, Seq<impl Numeric>), ()> {
    if rows <= 0 || cols <= 0 || rows * cols != seq.len() {
        Result<(Dims, Seq<impl Numeric>)::Err(())
    } else {
        Result<(Dims, Seq<impl Numeric>)::Ok(((rows, cols), seq))
    }
}
```
Which makes rust complain...
