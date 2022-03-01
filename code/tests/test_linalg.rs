use hacspec_lib::prelude::*;
use hacspec_linalg::*;

#[test]
fn test_1() {
	let t = hacspec_function(false);
	assert_eq!(t, 10)
}
