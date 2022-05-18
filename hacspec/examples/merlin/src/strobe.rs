//#![allow(dead_code)]
//#![allow(unused_imports)]
//#![allow(non_snake_case)]
//#![allow(unused_astrobeignments)]
//#![allow(unused_variables)]

use hacspec_lib::*;
use hacspec_sha3::*;
use hacspec_ristretto::*;

//consts
const STROBE_R: u8 = 166u8;

const FLAG_I: u8 = 1u8;
const FLAG_A: u8 = 1u8 << 1;
const FLAG_C: u8 = 1u8 << 2;
const FLAG_T: u8 = 1u8 << 3;
const FLAG_M: u8 = 1u8 << 4;
const FLAG_K: u8 = 1u8 << 5;

//state, pos, pos_begin, cur_fl
pub type Strobe = (StateU8, u8, u8, u8);

type StateU64 = State;
array!(StateU8, 200, U8);

pub fn new_strobe(protocol_label: Seq<U8>) -> Strobe {
	let mut st = StateU8::new();

	st = st.set_chunk(6,0,&byte_seq!(1u8, STROBE_R + 2, 1, 0, 1, 96));
	// b"STROBEv1.0.2"
	st = st.set_chunk(6,1,&byte_seq!(83u8,  84u8, 82u8, 79u8, 66u8, 69u8));
	st = st.set_chunk(6,2,&byte_seq!(118u8, 49u8, 46u8, 48u8, 46u8, 50u8));

	let st_U64 = transmute_state_to_u64(st);
	st = transmute_state_to_u8(keccakf1600(st_U64));

	meta_ad((st, 0u8, 0u8, 0u8), protocol_label, false)
}

fn transmute_state_to_u64(state: StateU8) -> StateU64 {
	let mut new_state = StateU64::new();

	for i in 0..new_state.len() {
		let mut word = U64Word::new();
		for j in 0..word.len() {
			word[j] = state[i*8 + j];
		}
		new_state[i] = U64_from_le_bytes(word);
	}

  for i in 0..new_state.len() {
  }

	new_state
}

fn transmute_state_to_u8(state: StateU64) -> StateU8 {
	let mut new_state = StateU8::new();

	for i in 0..state.len() {
		let bytes = state[i].to_le_bytes();
		for j in 0..bytes.len() {
			new_state[i*8+j] = bytes[j]
		}
	}

	new_state
}

fn run_f(mut strobe: Strobe) -> Strobe {
	let (mut state, mut pos, mut pos_begin, cur_fl) = strobe;

	state[pos] = state[pos] ^ U8::classify(pos_begin);
	state[pos + 1u8] = state[pos + 1u8] ^ U8::classify(0x04u8);
	state[STROBE_R + 1u8] = state[STROBE_R + 1u8] ^ U8::classify(0x80u8);
	let state_U64 = transmute_state_to_u64(state);
	state = transmute_state_to_u8(keccakf1600(state_U64));

	pos = 0u8;
	pos_begin = 0u8;

	(state, pos, pos_begin, cur_fl)
}

fn absorb(mut strobe: Strobe, data: Seq::<U8>) -> Strobe {
	let (mut state, mut pos, mut pos_begin, mut cur_fl) = strobe;

	for i in 0..data.len() {
		state[pos] = state[pos] ^ data[i];
		pos = pos + 1u8;
		if pos == STROBE_R {
			let (s, p, pb, cf) = run_f((state.clone(), pos, pos_begin, cur_fl));
			state = s;
			pos = p;
			pos_begin = pb;
			cur_fl = cf;
		}
	}

	(state, pos, pos_begin, cur_fl)
}

fn squeeze(mut strobe: Strobe, mut data: Seq<U8>) -> (Strobe, Seq<U8>) {
	let (mut state, mut pos, mut pos_begin, mut cur_fl) = strobe;

	for i in 0..data.len() {
		data[i] = state[pos];
		state[pos] = U8::classify(0u8);
		pos = pos + 1u8;
		if pos == STROBE_R {
			let (s, p, pb, cf) = run_f((state.clone(), pos, pos_begin, cur_fl));
			state = s;
			pos = p;
			pos_begin = pb;
			cur_fl = cf;
		}
	}

	((state, pos, pos_begin, cur_fl), data)
}

fn begin_op(mut strobe: Strobe, flags: u8, more: bool) -> Strobe {
	let (mut state, mut pos, mut pos_begin, mut cur_fl) = strobe;

	if more {
		strobe
	} else {
		let old_begin = pos_begin;

		pos_begin = pos + 1u8;
		cur_fl = flags;

		let mut data = Seq::<U8>::new(2);
		data[0usize] = U8::classify(old_begin);
		data[1usize] = U8::classify(flags);

		let (s, p, pb, cf) = absorb((state.clone(), pos, pos_begin, cur_fl), data);
		state = s;
		pos = p;
		pos_begin = pb;
		cur_fl = cf;

		// Force running F if C or K is set
		let force_f = 0u8 != (flags & (FLAG_C | FLAG_K));

		if force_f && pos != 0u8 {
			run_f((state.clone(), pos, pos_begin, cur_fl))
		}
		else {
			(state, pos, pos_begin, cur_fl)
		}
	}
}

pub fn meta_ad(mut strobe: Strobe, data: Seq<U8>, more: bool) -> Strobe {
	let (mut state, mut pos, mut pos_begin, mut cur_fl) = strobe;

	strobe = begin_op(strobe, FLAG_M | FLAG_A, more);
	strobe = absorb(strobe, data);

	strobe
}

pub fn ad(mut strobe: Strobe, data: Seq<U8>, more: bool) -> Strobe {
	let (mut state, mut pos, mut pos_begin, mut cur_fl) = strobe;

	strobe = begin_op(strobe, FLAG_A, more);
	strobe = absorb(strobe, data);

	strobe
}

pub fn prf(mut strobe: Strobe, data: Seq<U8>, more: bool) -> (Strobe, Seq<U8>) {
	let (mut state, mut pos, mut pos_begin, mut cur_fl) = strobe;

	strobe = begin_op(strobe, FLAG_I | FLAG_A | FLAG_C, more);
	squeeze(strobe, data)
}
