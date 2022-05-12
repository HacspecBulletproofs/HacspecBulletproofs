#![allow(non_snake_case)]

/*
* A hacspec Ristretto implementation modelled on the curve25519_dalek rust library.
* Functions are modelled and tested against their dalek counterparts
* using Quickcheck.

* This ensures, with reasonable probability, that the
* these functions and the dalek functions work identically. With this
* assumption, properties about the dalek library can be proven in
* hacspec target languages, and allows hacspec implementations to use
* the defined ristretto operations.
*
* Each internal representation of a point is kept in its Twisted Edwards 
* representation, while each encoded point is a byte-string of length 32.
*
* Each public function in the library is based on the IETF-standard for Ristretto
* while all helper functions are private. It is also important to note that 
* the internal representation of each point is kept hidden and inaccessible 
* to the outside in order to avoid giving incorrect encodings.
*/

use hacspec_lib::*;

public_nat_mod!(
    type_name: FieldElement,
    type_of_canvas: FieldCanvas,
    bit_size_of_field: 256,
    modulo_value: "7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed"
);

//Scalar is used only in decode to ensure the decoding is valid.
public_nat_mod!(
    type_name: Scalar,
    type_of_canvas: ScalarCanvas,
    bit_size_of_field: 256,
    modulo_value: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
);

pub type RistrettoPoint = (FieldElement, FieldElement, FieldElement, FieldElement);

bytes!(RistrettoPointEncoded, 32);

//Bytestrings are used as the input of the one-way-map
bytes!(ByteString, 64);

//Constants as defined by the IETF standard.

fn P() -> FieldElement {
    FieldElement::from_byte_seq_be(&byte_seq!(
        0x7fu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8,
        0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8,
        0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xedu8
    ))
}

fn D() -> FieldElement {
    FieldElement::from_byte_seq_be(&byte_seq!(
        0x52u8, 0x03u8, 0x6cu8, 0xeeu8, 0x2bu8, 0x6fu8, 0xfeu8, 0x73u8, 0x8cu8, 0xc7u8, 0x40u8,
        0x79u8, 0x77u8, 0x79u8, 0xe8u8, 0x98u8, 0x00u8, 0x70u8, 0x0au8, 0x4du8, 0x41u8, 0x41u8,
        0xd8u8, 0xabu8, 0x75u8, 0xebu8, 0x4du8, 0xcau8, 0x13u8, 0x59u8, 0x78u8, 0xa3u8
    ))
}

fn SQRT_M1() -> FieldElement {
    FieldElement::from_byte_seq_be(&byte_seq!(
        0x2bu8, 0x83u8, 0x24u8, 0x80u8, 0x4fu8, 0xc1u8, 0xdfu8, 0x0bu8, 0x2bu8, 0x4du8, 0x00u8,
        0x99u8, 0x3du8, 0xfbu8, 0xd7u8, 0xa7u8, 0x2fu8, 0x43u8, 0x18u8, 0x06u8, 0xadu8, 0x2fu8,
        0xe4u8, 0x78u8, 0xc4u8, 0xeeu8, 0x1bu8, 0x27u8, 0x4au8, 0x0eu8, 0xa0u8, 0xb0u8
    ))
}

fn INVSQRT_A_MINUS_D() -> FieldElement {
    FieldElement::from_byte_seq_be(&byte_seq!(
        0x78u8, 0x6cu8, 0x89u8, 0x05u8, 0xcfu8, 0xafu8, 0xfcu8, 0xa2u8, 0x16u8, 0xc2u8, 0x7bu8,
        0x91u8, 0xfeu8, 0x01u8, 0xd8u8, 0x40u8, 0x9du8, 0x2fu8, 0x16u8, 0x17u8, 0x5au8, 0x41u8,
        0x72u8, 0xbeu8, 0x99u8, 0xc8u8, 0xfdu8, 0xaau8, 0x80u8, 0x5du8, 0x40u8, 0xeau8
    ))
}

fn SQRT_AD_MINUS_ONE() -> FieldElement {
    FieldElement::from_byte_seq_be(&byte_seq!(
        0x37u8, 0x69u8, 0x31u8, 0xbfu8, 0x2bu8, 0x83u8, 0x48u8, 0xacu8, 0x0fu8, 0x3cu8, 0xfcu8,
        0xc9u8, 0x31u8, 0xf5u8, 0xd1u8, 0xfdu8, 0xafu8, 0x9du8, 0x8eu8, 0x0cu8, 0x1bu8, 0x78u8,
        0x54u8, 0xbdu8, 0x7eu8, 0x97u8, 0xf6u8, 0xa0u8, 0x49u8, 0x7bu8, 0x2eu8, 0x1bu8
    ))
}

fn ONE_MINUS_D_SQ() -> FieldElement {
    FieldElement::from_byte_seq_be(&byte_seq!(
        0x02u8, 0x90u8, 0x72u8, 0xa8u8, 0xb2u8, 0xb3u8, 0xe0u8, 0xd7u8, 0x99u8, 0x94u8, 0xabu8,
        0xddu8, 0xbeu8, 0x70u8, 0xdfu8, 0xe4u8, 0x2cu8, 0x81u8, 0xa1u8, 0x38u8, 0xcdu8, 0x5eu8,
        0x35u8, 0x0fu8, 0xe2u8, 0x7cu8, 0x09u8, 0xc1u8, 0x94u8, 0x5fu8, 0xc1u8, 0x76u8
    ))
}

fn D_MINUS_ONE_SQ() -> FieldElement {
    FieldElement::from_byte_seq_be(&byte_seq!(
        0x59u8, 0x68u8, 0xb3u8, 0x7au8, 0xf6u8, 0x6cu8, 0x22u8, 0x41u8, 0x4cu8, 0xdcu8, 0xd3u8,
        0x2fu8, 0x52u8, 0x9bu8, 0x4eu8, 0xebu8, 0xd2u8, 0x9eu8, 0x4au8, 0x2cu8, 0xb0u8, 0x1eu8,
        0x19u8, 0x99u8, 0x31u8, 0xadu8, 0x5au8, 0xaau8, 0x44u8, 0xedu8, 0x4du8, 0x20u8
    ))
}


//Special points needed for certain computations.

pub fn BASE_POINT_ENCODED() -> RistrettoPointEncoded {
    RistrettoPointEncoded::from_seq(&byte_seq!(
        0xe2u8,0xf2u8,0xaeu8,0x0au8,0x6au8,0xbcu8,0x4eu8,0x71u8,
        0xa8u8,0x84u8,0xa9u8,0x61u8,0xc5u8,0x00u8,0x51u8,0x5fu8,
        0x58u8,0xe3u8,0x0bu8,0x6au8,0xa5u8,0x82u8,0xddu8,0x8du8,
        0xb6u8,0xa6u8,0x59u8,0x45u8,0xe0u8,0x8du8,0x2du8,0x76u8
    ))
}
pub fn BASE_POINT() -> RistrettoPoint {
	decode(BASE_POINT_ENCODED()).unwrap()
}
pub fn IDENTITY_POINT() -> RistrettoPoint {
    (fe(0u128), fe(1u128), fe(1u128), fe(0u128))
}

// === Helper functions ===

//Creates a field element from the given literal.
fn fe(x: u128) -> FieldElement {
    FieldElement::from_literal(x)
}

//Checks if a given field element is negative. A negative field element is defined as an odd number.
fn IS_NEGATIVE(e: FieldElement) -> bool {
    e % fe(2u128) == fe(1u128)
}

//Checks if two given field elements are equal.
fn CT_EQ(u: FieldElement, v: FieldElement) -> bool {
    u == v
}

//given a condition it selects u if the condition is true and v if it is false.
fn CT_SELECT(u: FieldElement, cond: bool, v: FieldElement) -> FieldElement {
    if cond {
        u
    } else {
        v
    }
}

//Computes the additive negation of the given field element.
fn neg_fe(u: FieldElement) -> FieldElement {
    fe(0u128) - u
}

//returns the absolute value of the given field element. 
fn CT_ABS(u: FieldElement) -> FieldElement {
    CT_SELECT(neg_fe(u), IS_NEGATIVE(u), u)
}


//Computes if the division of the two given field elements is square and returns said square.
//This function has four different cases it can return with.
//1: if u, the numerator is 0 it returns (true,0).
//2: if v, the denominator is 0 it returns (false, 0) as you cannot divide by 0.
//3: if both are non-zero and u/v is square it returns (true, square).
//4: if both are non-zero and u/v is not square it returns (false, SQRT_M1*(u/v)).
fn SQRT_RATIO_M1(u: FieldElement, v: FieldElement) -> (bool, FieldElement) {
    let v3 = v.pow(2u128) * v;
    let v7 = v3.pow(2u128) * v;
    let mut r = (u * v3) * (u * v7).pow_felem((P() - fe(5u128)) / fe(8u128));
    let check = v * r.pow(2u128);

    let correct_sign_sqrt = CT_EQ(check, u);
    let flipped_sign_sqrt = CT_EQ(check, neg_fe(u));
    let flipped_sign_sqrt_i = CT_EQ(check, neg_fe(u) * SQRT_M1());

    let r_prime = SQRT_M1() * r;
    r = CT_SELECT(r_prime, flipped_sign_sqrt || flipped_sign_sqrt_i, r);

    // Choose the nonnegative square root.
    r = CT_ABS(r);

    let was_square = correct_sign_sqrt || flipped_sign_sqrt;

    (was_square, r)
}

//Takes a uniformly distributed Bytestring of length 64.
//Returns a pseudo-randomly generated Ristretto point using the defined IETF standard.
//While this function is not used for any computations later on it is useful for generating points.
pub fn one_way_map(b: ByteString) -> RistrettoPoint {
    let P1_bytes = b.slice(0,32);
    let P2_bytes = b.slice(32,32);

    let mut P1_bytes = P1_bytes.declassify();
    let mut P2_bytes = P2_bytes.declassify();

    P1_bytes[31] = P1_bytes[31] % 128u8;
    P2_bytes[31] = P2_bytes[31] % 128u8;

    let P1_field = FieldElement::from_public_byte_seq_le(P1_bytes);
    let P2_field = FieldElement::from_public_byte_seq_le(P2_bytes);

    let P1 = MAP(P1_field);
    let P2 = MAP(P2_field);

    add(P1,P2)
}

//A helper function for the one-way-map function. 
//Placed here as it is only used here and is used immedietely before returning.
//computes a ristretto point using the IETF standard on the given field element.
fn MAP(t: FieldElement) -> RistrettoPoint {
    let one = fe(1u128);
    let minus_one = neg_fe(one);
    let r = SQRT_M1() * t.pow(2u128);
    let u = (r + one) * ONE_MINUS_D_SQ();
    let v = (minus_one - r*D()) * (r + D());

    let (was_square, mut s) = SQRT_RATIO_M1(u, v);
    let s_prime = neg_fe(CT_ABS(s*t));
    s = CT_SELECT(s, was_square, s_prime);
    let c = CT_SELECT(minus_one, was_square, r);

    let N = c * (r - one) * D_MINUS_ONE_SQ() - v;

    let w0 = fe(2u128) * s * v;
    let w1 = N * SQRT_AD_MINUS_ONE();
    let w2 = one - s.pow(2u128);
    let w3 = one + s.pow(2u128);
    (w0*w3,w2*w1,w1*w3,w0*w2)
}


//Decodes the given point in accordance with the IETF standard. 
//Note: There are many byte-strings resulting in incorrect decodings. 
//These are all checked for, once more in accordance with the IETF standards.
pub fn decode(u: RistrettoPointEncoded) -> Result<RistrettoPoint, ()> {
    let mut ret = Result::<RistrettoPoint, ()>::Err(());
    let temp_s = Scalar::from_byte_seq_le(u);
    let p_as_s = Scalar::from_byte_seq_be(&byte_seq!(
        0x7fu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8,
        0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8,
        0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xedu8
    ));
    let s = FieldElement::from_byte_seq_le(u);

    if temp_s < p_as_s && !IS_NEGATIVE(s) {
        let one = fe(1u128);
        let ss = s.pow(2u128);
        let u1 = one - ss;
        let u2 = one + ss;
        let u2_sqr = u2.pow(2u128);

        let v = neg_fe(D() * u1.pow(2u128)) - u2_sqr;

        let (was_square, invsqrt) = SQRT_RATIO_M1(one, v * u2_sqr);

        let den_x = invsqrt * u2;
        let den_y = invsqrt * den_x * v;

        let x = CT_ABS((s + s) * den_x);
        let y = u1 * den_y;
        let t = x * y;

        if !(!was_square || IS_NEGATIVE(t) || y == fe(0u128)) {
            ret = Result::<RistrettoPoint, ()>::Ok((x, y, one, t));
        }
    }
    ret
}

//Encodes the given point to its encoded equivalent in accordance with the IETF standard.
pub fn encode(u: RistrettoPoint) -> RistrettoPointEncoded {
    let (x0, y0, z0, t0) = u;

    let u1 = (z0 + y0) * (z0 - y0);
    let u2 = x0 * y0;

    // Ignore was_square since this is always square
    let (_, invsqrt) = SQRT_RATIO_M1(fe(1u128), u1 * u2.pow(2u128));

    let den1 = invsqrt * u1;
    let den2 = invsqrt * u2;
    let z_inv = den1 * den2 * t0;

    let ix0 = x0 * SQRT_M1();
    let iy0 = y0 * SQRT_M1();
    let enchanted_denominator = den1 * INVSQRT_A_MINUS_D();

    let rotate = IS_NEGATIVE(t0 * z_inv);

    let x = CT_SELECT(iy0, rotate, x0);
    let mut y = CT_SELECT(ix0, rotate, y0);
    let z = z0;
    let den_inv = CT_SELECT(enchanted_denominator, rotate, den2);

    y = CT_SELECT(neg_fe(y), IS_NEGATIVE(x * z_inv), y);

    let s = CT_ABS(den_inv * (z - y));

    RistrettoPointEncoded::new().update_start(&s.to_byte_seq_le())
}

//Checks that two points are equivalent, in accordance with the definition given by the IETF standard.
pub fn equals(u: RistrettoPoint, v: RistrettoPoint) -> bool {
    let (x1, y1, _, _) = u;
    let (x2, y2, _, _) = v;
    x1 * y2 == x2 * y1 || y1 * y2 == x1 * x2
}

//adds two points together.
pub fn add(u: RistrettoPoint, v: RistrettoPoint) -> RistrettoPoint {
    let d = D();
    let (X1, Y1, Z1, T1) = u;
    let (X2, Y2, Z2, T2) = v;

    let A = (Y1 - X1) * (Y2 - X2);
    let B = (Y1 + X1) * (Y2 + X2);
    let C = T1 * fe(2u128) * d * T2;
    let D = Z1 * fe(2u128) * Z2;
    let E = B - A;
    let F = D - C;
    let G = D + C;
    let H = B + A;
    let X3 = E * F;
    let Y3 = G * H;
    let T3 = E * H;
    let Z3 = F * G;

    (X3, Y3, Z3, T3)
}

//Doubles the given point.
pub fn double(u: RistrettoPoint) -> RistrettoPoint {
    let (X1, Y1, Z1, _) = u;

    let A = X1.pow(2u128);
    let B = Y1.pow(2u128);
    let C = fe(2u128) * (Z1.pow(2u128));
    let H = A + B;
    let E = H - ((X1 + Y1).pow(2u128));
    let G = A - B;
    let F = C + G;
    let X2 = E * F;
    let Y2 = G * H;
    let T2 = E * H;
    let Z2 = F * G;

    (X2, Y2, Z2, T2)
}

//computes the negation of the given point.
pub fn neg(u: RistrettoPoint) -> RistrettoPoint {
    let (X1, Y1, Z1, T1) = u;
    (neg_fe(X1), Y1, neg_fe(Z1), T1)
}

//Subtracts v from u, using negation on v and adding them.
pub fn sub(u: RistrettoPoint, v: RistrettoPoint) -> RistrettoPoint {
    add(u, neg(v))
}

//performs scalar multiplication.
pub fn mul(k: FieldElement, p: RistrettoPoint) -> RistrettoPoint {
    let mut acc = IDENTITY_POINT();
    let mut q = p;
    for i in 0..256 - leading_zeros(k) {
        if k.get_bit(i) == fe(1u128) {
            acc = add(acc, q)
        }
        q = double(q)
    }
    acc
}

//computes the leading zeroes of the given field element. 
//This is only used for point multiplication above
fn leading_zeros(k: FieldElement) -> usize {
    let mut acc = 256usize;
    let mut done = false;
    for i in 0..256 {
        if !done && k.get_bit(256-i-1) == fe(1u128) {
            done = true;
            acc = i-1;
        }
    }
    acc
}