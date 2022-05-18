/*
 * This is a subset of Merlin...
 */

//#![allow(dead_code)]
//#![allow(unused_imports)]
//#![allow(non_snake_case)]
//#![allow(unused_assignments)]
//#![allow(unused_variables)]

mod strobe;
use hacspec_lib::*;
use crate::strobe::*;

pub type Transcript = Strobe;

pub fn new(init: Seq<U8>) -> Transcript {
	new_strobe(init)
}

pub fn encode_U64(x: U64) -> Seq<U8> {
	U64_to_le_bytes(x).to_le_bytes()
}

pub fn encode_usize_as_u32(x: usize) -> Seq<U8> {
	let x_U32 = U32::classify(x as u32);
	U32_to_le_bytes(x_U32).to_le_bytes()
}

// Strobe op: meta-AD(label || len(message)); AD(message)
pub fn append_message(mut transcript: Transcript, label: Seq<U8>, message: Seq<U8>) -> Transcript {
	let l = message.len();
	let data_len = U32_to_le_bytes(U32::classify(message.len() as u32)).to_be_bytes();
	transcript = meta_ad(transcript, label, false);
	transcript = meta_ad(transcript, data_len, true);
	transcript = ad(transcript, message, false);
	transcript
}

pub fn challenge_bytes(mut transcript: Transcript, label: Seq<U8>, dest: Seq<U8>) -> (Transcript, Seq<U8>) {
	let prf_len = dest.len();

	// metadata = label || len(challenge_bytes);
	let mut metadata: Seq<U8> = Seq::<U8>::new(label.len() + 4);
	metadata.concat(&label);
	metadata.concat(&encode_usize_as_u32(prf_len));

	transcript = meta_ad(transcript, metadata, false);
	prf(transcript, dest, false)
}

// Strobe op: meta-AD(label || len(message)); AD(message)
pub fn append_u64(mut transcript: Transcript, label: Seq<U8>, x: U64) -> Transcript {
	append_message(transcript, label, encode_U64(x))
}
