#![allow(dead_code)]
use hacspec_lib::*;
use hacspec_ristretto::*;

pub type PedersenGens = ( 
    /*Base point:*/ RistrettoPoint, 
    /*Blinding point:*/ RistrettoPoint
);

pub type BulletproofGens = (
    /* Party capacity:*/ usize,
    /*Gens capacity:*/ usize,
    /*Precomputed G-generators:*/ 
    /* g_vec: */Seq<Seq<RistrettoPoint>>, 
    /*Precomputed H-generators for each party:*/ 
    /* h_vec: */Seq<Seq<RistrettoPoint>>
);

pub type ProofShare = (
    /*t_x:*/ Scalar,
    /*t_x_blinding:*/ Scalar,
    /*e_blinding:*/ Scalar,
    /*l_vec:*/ Seq<Scalar>,
    /*r_vec:*/ Seq<Scalar>
);