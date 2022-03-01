use hacspec_lib::*;

//struct Matrix (Seq::<Seq::<bool>>);

pub fn hacspec_function(x: bool) -> bool {
	let mut a = Seq::<Seq::<bool>>::new(10);
	let mut b = Seq::<bool>::new(10);

	b[2] = true;

	//a[0] = Seq::<bool>::new(10);
	//a[0][0] = true;
	//a[0][0]

	b[2]
}
