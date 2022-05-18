//#![allow(dead_code)]
//#![allow(unused_imports)]
//#![allow(non_snake_case)]
//#![allow(unused_atranscriptignments)]
//#![allow(unused_variables)]

use hacspec_lib::*;
//use hacspec_sha3::*;
use hacspec_ristretto::*;
use hacspec_merlin::*;

public_nat_mod!(
    type_name: LocalFieldElement,
    type_of_canvas: FieldCanvas,
    bit_size_of_field: 512,
    modulo_value: "7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed"
);

pub fn innerproduct_domain_sep(mut transcript: Transcript, n: U64) -> Transcript {
	let place_holder = Seq::<U8>::new(0);
	//"dom_sep"
	let dom_sep = byte_seq!(100, 111, 109, 45, 115, 101, 112);
	//"ipp v1"
	let ipp_v1 = byte_seq!(105, 112, 112, 32, 118, 49);
	//"n"
	let n_ = byte_seq!(110);

	transcript = append_message(transcript, dom_sep, ipp_v1);
	transcript = append_u64(transcript, n_, n);

	transcript
}

pub fn challenge_scalar(transcript: Transcript, label: Seq<U8>) -> (Transcript, FieldElement) {
	let buf = Seq::<U8>::new(64);
	let (new_transcript, data) = challenge_bytes(transcript, label, buf);
	println!("{}", data.len());
	let fe = LocalFieldElement::from_byte_seq_le(data);
	let fe_ = FieldElement::from_byte_seq_le(fe.to_byte_seq_le().slice(0, 32));
	(new_transcript, fe_)
}

pub fn append_point(mut transcript: Transcript, label: Seq<U8>, point: RistrettoPointEncoded) -> Transcript {
	append_message(transcript, label, point.to_le_bytes())
}
