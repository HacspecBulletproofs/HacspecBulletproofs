//TODO: Rewrite seqs to vectors/matrices
use hacspec_lib::*;

type State = Seq::<u64>;

const RC: &'static [usize] = &[
  0x0000000000000001, 0x0000000000008082, 0x800000000000808a,
  0x8000000080008000, 0x000000000000808b, 0x0000000080000001,
  0x8000000080008081, 0x8000000000008009, 0x000000000000008a,
  0x0000000000000088, 0x0000000080008009, 0x000000008000000a,
  0x000000008000808b, 0x800000000000008b, 0x8000000000008089,
  0x8000000000008003, 0x8000000000008002, 0x8000000000000080,
  0x000000000000800a, 0x800000008000000a, 0x8000000080008081,
  0x8000000000008080, 0x0000000080000001, 0x8000000080008008
];

const RX: &'static [u32] = &[
	0, 1, 62, 28, 27,
	36, 44, 6, 55, 20,
	3, 10, 43, 25, 39,
	41, 45, 15, 21, 8,
	18, 2, 61, 56, 14
];

fn theta(mut state: State) -> State {
	let mut C = Seq::<u64>::new(5);
	let mut D = Seq::<u64>::new(5);

	for x in 0..5 {
		C[x] = (state[0+x] ^ state[5+x] ^ state[10+x] ^ state[15+x] ^ state[20+x]) as u64;
	}

	for x in 0..5 {
		// in order to avoid negative mod values, we've replaced "(x - 1) % 5" with "(x + 4) % 5"
		D[x] = C[(x + 4) % 5] ^ (C[(x + 1) % 5] as u64).rotate_left(1);

		for y in 0..5 {
			state[y * 5 + x] = state[y * 5 + x] ^ D[x];
		}
	}
	state
}

fn rho(mut state: State) -> State {
	for y in 0..5 {
		for x in 0..5 {
			state[y * 5 + x] = (state[y * 5 + x] as u64).rotate_left(RX[y * 5 + x])
		}
	}

	state
}

fn pi(mut state: State) -> State {
	let mut B = Seq::<u64>::new(25);
	for y in 0..5 {
		for x in 0..5 {
			B[y * 5 + x] = state[5 * y + x];
		}
	}
	for y in 0..5 {
		for x in 0..5 {
			let u = (0 * x + 1 * y) % 5;
			let v = (2 * x + 3 * y) % 5;

			state[v * 5 + u] = B[5 * y + x];
		}
	}

	state
}

fn chi(mut state: State) -> State {
	let mut C = Seq::<u64>::new(5);
	for y in 0..5 {
		for x in 0..5 {
			C[x] = (state[y * 5 + x] as u64) ^ ((!(state[y * 5 + ((x+1) % 5)] as u64)) & state[y * 5 + ((x + 2) % 5)]);
		}
		
		for x in 0..5 {
			state[y * 5 + x] = C[x];
		}
	}

	state
}

fn iota(mut state: State, i: usize) -> State {
	state[0] = state[0] ^ (RC[i] as u64);
	state
}

pub fn f1600(mut state: State) -> State {
	let rounds = 24;

	for i in 0..rounds {
		state = theta(state);
		state = rho(state);
		state = pi(state);
		state = chi(state);
		state = iota(state, i);
	}

	state
}
