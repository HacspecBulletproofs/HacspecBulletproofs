extern crate quickcheck;

use quickcheck::*;

use hacspec_lib::*;
use hacspec_keccak::*;

// === Helper functions ===

fn quickcheck(helper: impl Testable) {
    QuickCheck::new()
        .tests(100)
        .min_tests_passed(100)
        .max_tests(1000000)
        .quickcheck(helper);
}

#[test]
// Test vectors are from KeccakCodePackage
fn test() {
	let mut data = Seq::<u64>::new(25);

	let kek1 = Seq::<u64>::from_vec(vec![
		0xF1258F7940E1DDE7, 0x84D5CCF933C0478A, 0xD598261EA65AA9EE, 0xBD1547306F80494D,
		0x8B284E056253D057, 0xFF97A42D7F8E6FD4, 0x90FEE5A0A44647C4, 0x8C5BDA0CD6192E76,
		0xAD30A6F71B19059C, 0x30935AB7D08FFC64, 0xEB5AA93F2317D635, 0xA9A6E6260D712103,
		0x81A57C16DBCF555F, 0x43B831CD0347C826, 0x01F22F1A11A5569F, 0x05E5635A21D9AE61,
		0x64BEFEF28CC970F2, 0x613670957BC46611, 0xB87C5A554FD00ECB, 0x8C3EE88A1CCF32C8,
		0x940C7922AE3A2614, 0x1841F924A2C509E4, 0x16F53526E70465C2, 0x75F644E97F30A13B,
		0xEAF1FF7B5CECA249,
	]);

	let kek2 = keccakf1600(data.clone());

	let kek3 = Seq::<u64>::from_vec(vec![
		0x2D5C954DF96ECB3C, 0x6A332CD07057B56D, 0x093D8D1270D76B6C, 0x8A20D9B25569D094,
		0x4F9C4F99E5E7F156, 0xF957B9A2DA65FB38, 0x85773DAE1275AF0D, 0xFAF4F247C3D810F7,
		0x1F1B9EE6F79A8759, 0xE4FECC0FEE98B425, 0x68CE61B6B9CE68A1, 0xDEEA66C4BA8F974F,
		0x33C43D836EAFB1F5, 0xE00654042719DBD9, 0x7CF8A9F009831265, 0xFD5449A6BF174743,
		0x97DDAD33D8994B40, 0x48EAD5FC5D0BE774, 0xE3B8C8EE55B7B03C, 0x91A0226E649E42E9,
		0x900E3129E7BADD7B, 0x202A9EC5FAA3CCE8, 0x5B3402464E1C3DB6, 0x609F4E62A44C1059,
		0x20D06CD26A8FBF5C,
	]);
	let kek4 = keccakf1600(kek2.clone());

	for i in 0..data.len() {
		assert_eq!(kek1[i], kek1[i])
	}

	for i in 0..data.len() {
		assert_eq!(kek3[i], kek4[i])
	}
}

#[test]
fn keccak_f1600() {
	// Test vectors are copied from XKCP (eXtended Keccak Code Package)
	// https://github.com/XKCP/XKCP/blob/master/tests/TestVectors/KeccakF-1600-IntermediateValues.txt
	let state_first = Seq::<u64>::from_vec(vec![
		0xF1258F7940E1DDE7,
		0x84D5CCF933C0478A,
		0xD598261EA65AA9EE,
		0xBD1547306F80494D,
		0x8B284E056253D057,
		0xFF97A42D7F8E6FD4,
		0x90FEE5A0A44647C4,
		0x8C5BDA0CD6192E76,
		0xAD30A6F71B19059C,
		0x30935AB7D08FFC64,
		0xEB5AA93F2317D635,
		0xA9A6E6260D712103,
		0x81A57C16DBCF555F,
		0x43B831CD0347C826,
		0x01F22F1A11A5569F,
		0x05E5635A21D9AE61,
		0x64BEFEF28CC970F2,
		0x613670957BC46611,
		0xB87C5A554FD00ECB,
		0x8C3EE88A1CCF32C8,
		0x940C7922AE3A2614,
		0x1841F924A2C509E4,
		0x16F53526E70465C2,
		0x75F644E97F30A13B,
		0xEAF1FF7B5CECA249,
	]);
	let state_second = Seq::<u64>::from_vec(vec![
		0x2D5C954DF96ECB3C,
		0x6A332CD07057B56D,
		0x093D8D1270D76B6C,
		0x8A20D9B25569D094,
		0x4F9C4F99E5E7F156,
		0xF957B9A2DA65FB38,
		0x85773DAE1275AF0D,
		0xFAF4F247C3D810F7,
		0x1F1B9EE6F79A8759,
		0xE4FECC0FEE98B425,
		0x68CE61B6B9CE68A1,
		0xDEEA66C4BA8F974F,
		0x33C43D836EAFB1F5,
		0xE00654042719DBD9,
		0x7CF8A9F009831265,
		0xFD5449A6BF174743,
		0x97DDAD33D8994B40,
		0x48EAD5FC5D0BE774,
		0xE3B8C8EE55B7B03C,
		0x91A0226E649E42E9,
		0x900E3129E7BADD7B,
		0x202A9EC5FAA3CCE8,
		0x5B3402464E1C3DB6,
		0x609F4E62A44C1059,
		0x20D06CD26A8FBF5C,
	]);
	for i in 0..state_first.len() {
		assert_eq!(keccakf1600(state_first.clone())[i], state_second[i])
	}
}
