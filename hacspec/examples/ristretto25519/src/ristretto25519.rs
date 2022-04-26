#![allow(non_snake_case)]

use hacspec_lib::*;
use num::BigUint;

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

fn P() -> FieldElement{ return FieldElement::from_hex("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed")}
fn D() -> FieldElement{ return FieldElement::from_hex("52036cee2b6ffe738cc740797779e89800700a4d4141d8ab75eb4dca135978a3")}
fn SQRT_M1() -> FieldElement{ return FieldElement::from_hex("2b8324804fc1df0b2b4d00993dfbd7a72f431806ad2fe478c4ee1b274a0ea0b0")}
fn SQRT_AD_MINUS_ONE() -> FieldElement { return FieldElement::from_hex("376931bf2b8348ac0f3cfcc931f5d1fdaf9d8e0c1b7854bd7e97f6a0497b2e1b")}
fn INVSQRT_A_MINUS_D() -> FieldElement { return FieldElement::from_hex("786c8905cfaffca216c27b91fe01d8409d2f16175a4172be99c8fdaa805d40ea")}
fn ONE_MINUS_D_SQ() -> FieldElement { return FieldElement::from_hex("29072a8b2b3e0d79994abddbe70dfe42c81a138cd5e350fe27c09c1945fc176")}
fn D_MINUS_ONE_SQ() -> FieldElement { FieldElement::from_hex("5968b37af66c22414cdcd32f529b4eebd29e4a2cb01e199931ad5aaa44ed4d20")}

type Point = (FieldElement,FieldElement,FieldElement,FieldElement);

bytes!(EncodedPoint,32);

pub fn IS_NEGATIVE(e: FieldElement) -> bool { //What the fuck??????????

    let bytes = e.to_public_byte_seq_le();

    (bytes[0u32] & 1u8) == 1u8
}

fn CT_EQ(u: FieldElement, v:FieldElement) -> bool {
    u==v
}

fn CT_SELECT(u: FieldElement, cond: bool, v: FieldElement) -> FieldElement {
    if cond { u }
    else { v }
}

fn CT_ABS(u: FieldElement) -> FieldElement { 
    CT_SELECT(neg(u),IS_NEGATIVE(u),u)
}

pub fn invert(u: FieldElement) -> FieldElement {
    if u == flit(0) { 
        return flit(0)
    }
    u.pow_felem(P()-flit(2))
}

pub fn neg(u: FieldElement) -> FieldElement {
    P()-u
}

pub fn flit(x: u128) -> FieldElement {
    FieldElement::from_literal(x)
}

pub fn SQRT_RATIO_M1(u: FieldElement, v: FieldElement) -> (bool, FieldElement) {

    let v3 = v.pow(2u128)*v;
    let v7 = v3.pow(2u128)*v;
    let mut r = (u * v3) * (u * v7).pow_felem((P()-flit(5))/flit(8));
    let check = v * r.pow(2u128);
    
    let correct_sign_sqrt = CT_EQ(check, u);
    let flipped_sign_sqrt = CT_EQ(check, neg(u));
    let flipped_sign_sqrt_i = CT_EQ(check, neg(u)*SQRT_M1());
    
    let r_prime = SQRT_M1()*r;
    r = CT_SELECT(r_prime, flipped_sign_sqrt || flipped_sign_sqrt_i, r);
    
    // Choose the nonnegative square root.
    r = CT_ABS(r);
    
    let was_square = correct_sign_sqrt || flipped_sign_sqrt;
    
    (was_square, r)
    
}

/*fn MAP() -> X25519SerializedPoint {

}*/

pub fn  decode(u: EncodedPoint) -> Result<Point, ()> { // if s is larger than p decoding should fail. But Fieldelement.    
    let temp_s = Scalar::from_byte_seq_le(u);
    let p_as_s = Scalar::from_hex("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed");
    if temp_s >= p_as_s {
        return Result::<Point, ()>::Err(())
    }
    let s = FieldElement::from_byte_seq_le(u);
    if IS_NEGATIVE(s) {
        return Result::<Point, ()>::Err(())
    }
    let ss = s.pow(2);
    let u1 = flit(1) - ss;
    let u2 = flit(1) + ss;
    let u2_sqr = u2.pow(2);

    let v = neg(D() * u1.pow(2)) - u2_sqr;

    let (was_square, invsqrt) = SQRT_RATIO_M1(flit(1), v * u2_sqr);

    let den_x = invsqrt * u2;
    let den_y = invsqrt * den_x * v;

    let x = CT_ABS((s + s) * den_x);
    let y = u1 * den_y;
    let t = x * y;

    let mut ret = Result::<Point, ()>::Ok((x,y,flit(1),t));

    if !was_square || IS_NEGATIVE(t) || y == flit(0) {
        ret = Result::<Point, ()>::Err(())
    }
    ret

}

pub fn encode(u: Point) -> EncodedPoint {
    let (x0,y0,z0,t0) = u;
    
    let u1 = (z0 + y0) * (z0 - y0);
    let u2 = x0 * y0;

    // Ignore was_square since this is always square
    let (_, invsqrt) = SQRT_RATIO_M1(flit(1), u1 * u2.pow(2));

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

    y = CT_SELECT(neg(y), IS_NEGATIVE(x * z_inv), y);

    let s = CT_ABS(den_inv * (z - y));

    EncodedPoint::new().update_start(&s.to_byte_seq_le())
    
}

pub fn equals(u: Point, v: Point) -> bool {
    let (x1,y1,_,_) = u;
    let (x2,y2,_,_) = v;
    x1*y2 ==x2*y1 || y1*y2 ==x1*x2
}

/*pub fn add_points(u: Point, v: Point) -> Point {
    
}

pub fn negate_point(u: Point) -> Point {

}

pub fn scalar_multiplication(s: Scalar, u: Point) -> Point {
    
}*/