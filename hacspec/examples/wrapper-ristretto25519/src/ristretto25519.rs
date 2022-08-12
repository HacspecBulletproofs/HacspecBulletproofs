#![allow(non_snake_case)]
/*
* A hacspec Ristretto implementation modelled on the curve25519_dalek rust library.
* Functions are modelled and tested against their dalek counterparts
* using Quickcheck.
*
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
*
* For more information see the aforementioned IETF-standard here:
* https://www.ietf.org/archive/id/draft-irtf-cfrg-ristretto255-00.html#name-negative-field-elements/
* And the ristretto documentation:
* https://ristretto.group/ristretto.html/
*/

use hacspec_lib::*;
use curve25519_dalek_ng as curve25519_dalek;
use curve25519_dalek::*;

// Ristretto points are represented here by Extended Twisted Edwards Coordinates:
// https://eprint.iacr.org/2008/522.pdf
pub type RistrettoPoint = curve25519_dalek_ng::ristretto::RistrettoPoint;
pub type RistrettoPointEncoded = curve25519_dalek_ng::ristretto::CompressedRistretto;

type DecodeResult = Result<RistrettoPoint, u8>;

// Bytestrings are used as the input of the one-way-map function.
bytes!(ByteString, 64);

pub type Scalar = curve25519_dalek_ng::scalar::Scalar;

pub trait ExtPoint {
    fn to_le_bytes(self) -> Seq<U8>; 
    fn from_public_array(arr: [u8;32]) -> Self;
}

impl ExtPoint for RistrettoPointEncoded {
    fn to_le_bytes(self) -> Seq<U8> {
        let mut seq = Seq::<U8>::new(32usize);
        let slice = self.to_bytes();
        for i in 0..32 {
            seq[i] = U8::classify(slice[i]);
        }
        seq
    }
    fn from_public_array(arr: [u8;32]) -> Self {

        RistrettoPointEncoded::from_slice(&arr)
    }

}

pub trait Ext {
    fn ZERO() -> Self;
    fn ONE() -> Self;
    fn inv(&self) -> Self;
    fn pow(&self, p: u128) -> Self;
    fn from_byte_seq_le(seq: Seq<U8>) -> Self;
    fn to_byte_seq_le(self) -> Seq<U8>;
    fn from_literal(u: u128) -> Self;
    fn to_le_bytes(self) -> Vec<u8>;
}

impl Ext for Scalar {
    fn ZERO() -> Self{
        Scalar::zero()
    }
    fn ONE() -> Self{
        Scalar::one()
    }
    fn inv(&self) -> Self{
        self.invert()
    }
    fn pow(&self, p: u128) -> Self{
        let mut res = Scalar::one();
        for _ in 0..(p as usize) {
            res = res * self;
        }
        res
    }
    fn from_byte_seq_le(seq: Seq<U8>) -> Self {

        let mut arr: [u8; 32] = [0; 32];

        for i in 0..32 {
            arr[i] = seq[i].declassify();
        }
        Scalar::from_bytes_mod_order(arr)
    }
    fn to_byte_seq_le(self) -> Seq<U8> {
        let arr = self.as_bytes();
        let mut seq = Seq::<U8>::new(32usize);

        for i in 0..32{
            seq[i] = U8::classify(arr[i])
        }
        seq
    }
    fn from_literal(u: u128) -> Self {
        Scalar::from(u)
    }
    fn to_le_bytes(self) -> Vec<u8> {
        let mut vec = Vec::<u8>::new();
        let bytes = self.as_bytes();
        for i in 0..32 {
            vec.push(bytes[i]);
        }
        vec
    }
}



// === Constants === //

const DECODING_ERROR: u8 = 10;

// === Special points === //

pub fn BASE_POINT_ENCODED() -> RistrettoPointEncoded {
    curve25519_dalek_ng::constants::RISTRETTO_BASEPOINT_COMPRESSED
}

pub fn BASE_POINT() -> RistrettoPoint {
    curve25519_dalek_ng::constants::RISTRETTO_BASEPOINT_POINT
}

pub fn IDENTITY_POINT() -> RistrettoPoint {
    BASE_POINT() - BASE_POINT()
}

// === Helper functions === //

fn seq_to_arr(xs: ByteString) -> [u8; 64] {
    let mut arr: [u8; 64] = [0; 64];
    for i in 0..xs.len() {
        arr[i] = xs[i].declassify()
    }
    arr
}

// Computes if the division of the two given field elements is square and returns said squa

// === External Functions === //



/// Takes a uniformly distributed Bytestring of length 64.
/// Returns a pseudo-randomly generated Ristretto point following the defined IETF standard.
/// While this function is not used for any point computations, it is useful for generating points.
pub fn one_way_map(b: ByteString) -> RistrettoPoint {

    RistrettoPoint::from_uniform_bytes(&(seq_to_arr(b)))
}

/// Decodes the given point in accordance with the IETF standard.
/// Note: There are many byte-strings resulting in incorrect decodings.
/// These all checked for, in accordance with the IETF standards.
pub fn decode(u: RistrettoPointEncoded) -> DecodeResult {
    u.decompress().ok_or(DECODING_ERROR)
}

/// Encodes the given point
pub fn encode(u: RistrettoPoint) -> RistrettoPointEncoded {
    u.compress()
}

/// Checks that two points are equivalent.
pub fn equals(u: RistrettoPoint, v: RistrettoPoint) -> bool {
    u == v
}

/// Adds two points together.
pub fn add(u: RistrettoPoint, v: RistrettoPoint) -> RistrettoPoint {
    u + v
}

/// Doubles the given point. Note, this is faster than
/// adding a point to itself.
pub fn double(u: RistrettoPoint) -> RistrettoPoint {
    u + u
}

/// Computes the negation of the given point.
pub fn neg(u: RistrettoPoint) -> RistrettoPoint {
    -u
}

/// Subtracts v from u, using negation on v and then adding.
pub fn sub(u: RistrettoPoint, v: RistrettoPoint) -> RistrettoPoint {
    u - v
}

/// Performs scalar multiplication on a point.
pub fn mul(k: Scalar, p: RistrettoPoint) -> RistrettoPoint {
    k * p
}
