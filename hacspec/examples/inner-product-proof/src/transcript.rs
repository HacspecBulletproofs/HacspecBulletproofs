//#![allow(dead_code)]
//#![allow(unused_imports)]
//#![allow(non_snake_case)]
//#![allow(unused_atranscriptignments)]
//#![allow(unused_variables)]

use hacspec_lib::*;
use hacspec_ristretto::*;
use hacspec_merlin::*;

const VERIFICATION_ERROR: u8 = 22u8;

nat_mod!(
    type_name: LocalScalar,
    type_of_canvas: ScalarCanvas,
    bit_size_of_field: 512,
    modulo_value: "1000000000000000000000000000000014def9dea2f79cd65812631a5cf5d3ed"
);

pub fn innerproduct_domain_sep(mut transcript: Transcript, n: U64) -> Transcript {
	//b"dom-sep"
	let dom_sep = byte_seq!(100u8, 111u8, 109u8, 45u8, 115u8, 101u8, 112u8);
	//b"ipp v1"
	let ipp_v1 = byte_seq!(105u8, 112u8, 112u8, 32u8, 118u8, 49u8);
	//b"n"
	let n_ = byte_seq!(110u8);

	transcript = append_message(transcript, dom_sep, ipp_v1);
	transcript = append_U64(transcript, n_, n);

	transcript
}

pub fn challenge_scalar(transcript: Transcript, label: Seq<U8>) -> (Transcript, Scalar) {
	let buf = Seq::<U8>::new(64);
	let (new_transcript, data) = challenge_bytes(transcript, label, buf);

	let fe = LocalScalar::from_byte_seq_le(data.clone());
	let fe_ = Scalar::from_byte_seq_le(fe.to_byte_seq_le().slice(0, 32));
	(new_transcript, fe_)
}

pub fn append_point(transcript: Transcript, label: Seq<U8>, point: RistrettoPointEncoded) -> Transcript {
	append_message(transcript, label, point.to_le_bytes())
}

pub fn validate_and_append_point(transcript: Transcript, label: Seq<U8>, point: RistrettoPointEncoded) -> Result<Transcript,u8> {
	if equals(IDENTITY_POINT(), decode(point).unwrap()) {
		Result::<Transcript,u8>::Err(VERIFICATION_ERROR)
	} else {
		Result::<Transcript,u8>::Ok(append_message(transcript, label, point.to_le_bytes()))
	}
}
