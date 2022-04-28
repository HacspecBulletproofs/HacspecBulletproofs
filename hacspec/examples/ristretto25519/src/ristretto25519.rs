#![allow(non_snake_case)]

use hacspec_lib::*;

public_nat_mod!(
    type_name: FieldElement,
    type_of_canvas: FieldCanvas,
    bit_size_of_field: 256,
    modulo_value: "7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed"
);

public_nat_mod!(
    type_name: Scalar,
    type_of_canvas: ScalarCanvas,
    bit_size_of_field: 256,
    modulo_value: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
);

pub type RistrettoPoint = (FieldElement, FieldElement, FieldElement, FieldElement);

bytes!(RistrettoPointEncoded, 32);

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
/*
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
*/
pub fn BASE_POINT_ENCODED() -> RistrettoPointEncoded {
	RistrettoPointEncoded::from_hex("e2f2ae0a6abc4e71a884a961c500515f58e30b6aa582dd8db6a65945e08d2d76")
}
pub fn BASE_POINT() -> RistrettoPoint {
	decode(RistrettoPointEncoded::from_hex("e2f2ae0a6abc4e71a884a961c500515f58e30b6aa582dd8db6a65945e08d2d76")).unwrap()
}
pub fn IDENTITY_POINT() -> RistrettoPoint {
    (flit(0), flit(1), flit(1), flit(0))
}

// === Helper functions ===

pub fn flit(x: u128) -> FieldElement {
    FieldElement::from_literal(x)
}

pub fn to_bytes(p: RistrettoPoint) -> Seq<U8> {
	let p_enc = encode(p);
	p_enc.to_le_bytes()
}

fn IS_NEGATIVE(e: FieldElement) -> bool {
    e % flit(2u128) == flit(1u128)
}

fn CT_EQ(u: FieldElement, v: FieldElement) -> bool {
    u == v
}

fn CT_SELECT(u: FieldElement, cond: bool, v: FieldElement) -> FieldElement {
    if cond {
        u
    } else {
        v
    }
}

fn CT_ABS(u: FieldElement) -> FieldElement {
    CT_SELECT(neg_elem(u), IS_NEGATIVE(u), u)
}

fn neg_elem(u: FieldElement) -> FieldElement {
    P() - u
}

fn SQRT_RATIO_M1(u: FieldElement, v: FieldElement) -> (bool, FieldElement) {
    let v3 = v.pow(2u128) * v;
    let v7 = v3.pow(2u128) * v;
    let mut r = (u * v3) * (u * v7).pow_felem((P() - flit(5u128)) / flit(8u128));
    let check = v * r.pow(2u128);

    let correct_sign_sqrt = CT_EQ(check, u);
    let flipped_sign_sqrt = CT_EQ(check, neg_elem(u));
    let flipped_sign_sqrt_i = CT_EQ(check, neg_elem(u) * SQRT_M1());

    let r_prime = SQRT_M1() * r;
    r = CT_SELECT(r_prime, flipped_sign_sqrt || flipped_sign_sqrt_i, r);

    // Choose the nonnegative square root.
    r = CT_ABS(r);

    let was_square = correct_sign_sqrt || flipped_sign_sqrt;

    (was_square, r)
}

//fn MAP() -> RistrettoX25519SerializedPoint {}

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
        let ss = s.pow(2u128);
        let u1 = flit(1u128) - ss;
        let u2 = flit(1u128) + ss;
        let u2_sqr = u2.pow(2u128);

        let v = neg_elem(D() * u1.pow(2u128)) - u2_sqr;

        let (was_square, invsqrt) = SQRT_RATIO_M1(flit(1u128), v * u2_sqr);

        let den_x = invsqrt * u2;
        let den_y = invsqrt * den_x * v;

        let x = CT_ABS((s + s) * den_x);
        let y = u1 * den_y;
        let t = x * y;

        if !(!was_square || IS_NEGATIVE(t) || y == flit(0u128)) {
            ret = Result::<RistrettoPoint, ()>::Ok((x, y, flit(1u128), t));
        }
    }
    ret
}

pub fn encode(u: RistrettoPoint) -> RistrettoPointEncoded {
    let (x0, y0, z0, t0) = u;

    let u1 = (z0 + y0) * (z0 - y0);
    let u2 = x0 * y0;

    // Ignore was_square since this is always square
    let (_, invsqrt) = SQRT_RATIO_M1(flit(1u128), u1 * u2.pow(2u128));

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

    y = CT_SELECT(neg_elem(y), IS_NEGATIVE(x * z_inv), y);

    let s = CT_ABS(den_inv * (z - y));

    RistrettoPointEncoded::new().update_start(&s.to_byte_seq_le())
}

pub fn equals(u: RistrettoPoint, v: RistrettoPoint) -> bool {
    let (x1, y1, _, _) = u;
    let (x2, y2, _, _) = v;
    x1 * y2 == x2 * y1 || y1 * y2 == x1 * x2
}

pub fn add(u: RistrettoPoint, v: RistrettoPoint) -> RistrettoPoint {
    let d = D();
    let (X1, Y1, Z1, T1) = u;
    let (X2, Y2, Z2, T2) = v;

    let A = (Y1 - X1) * (Y2 - X2);
    let B = (Y1 + X1) * (Y2 + X2);
    let C = T1 * flit(2u128) * d * T2;
    let D = Z1 * flit(2u128) * Z2;
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

pub fn double(u: RistrettoPoint) -> RistrettoPoint {
    let (X1, Y1, Z1, _) = u;

    let A = X1.pow(2u128);
    let B = Y1.pow(2u128);
    let C = flit(2u128) * (Z1.pow(2u128));
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

pub fn neg(u: RistrettoPoint) -> RistrettoPoint {
    let (X1, Y1, Z1, T1) = u;
    (neg_elem(X1), Y1, neg_elem(Z1), T1)
}

pub fn sub(u: RistrettoPoint, v: RistrettoPoint) -> RistrettoPoint {
    add(u, neg(v))
}

pub fn mul(k: FieldElement, p: RistrettoPoint) -> RistrettoPoint {
    let mut acc = IDENTITY_POINT();
    let mut q = p;
    for i in 0..256 {
        if k.get_bit(i) == flit(1) {
            acc = add(acc, q)
        }
        q = double(q)
    }
    acc
}

