#![allow(non_snake_case)]
mod transcript;
mod errors;
mod types;
use transcript::*;
use errors::*;
use types::*;

use hacspec_lib::*;
use hacspec_merlin::*;
use hacspec_ristretto::*;
use hacspec_ipp::*;
use hacspec_pedersen::*;

type CreatePartyRes = Result<PartyAwaitingPosition,u8>;
type AssignPositionRes = Result<(PartyAwaitingBitChallenge,(RistrettoPointEncoded,RistrettoPoint,RistrettoPoint)),u8>;
type CreatePolyCommitmentRes = Result<(PartyAwaitingPolyChallenge,(RistrettoPoint,RistrettoPoint)), u8>;
type CreateProofShareRes = Result<ProofShare,u8>;



pub type PartyAwaitingPosition = (
    /*bp_gens:*/ BulletproofGens,
    /*pc_gens:*/ PedersenGens,
    /*n:*/ usize,
    /*v:*/ u64,
    /*v_blinding:*/ Scalar,
    /*V:*/ RistrettoPointEncoded
);

pub type PartyAwaitingBitChallenge = (
        /*n:*/ usize, /* bitsize of the range*/
        /*v:*/ u64,
        /*v_blinding:*/ Scalar,
        /*j:*/ usize,
        /*pc_gens:*/ PedersenGens,
        /*a_blinding:*/ Scalar,
        /*s_blinding:*/ Scalar,
        /*s_L:*/ Seq<Scalar>,
        /*s_R:*/ Seq<Scalar>
);

pub type PartyAwaitingPolyChallenge = (
/*offset_zz:*/ Scalar,
/*l_poly:*/ (Seq<Scalar>, Seq<Scalar>),
/*r_poly:*/ (Seq<Scalar>, Seq<Scalar>),
/*t_poly:*/ (Scalar, Scalar, Scalar),
/*v_blinding:*/ Scalar,
/*a_blinding:*/ Scalar,
/*s_blinding:*/ Scalar,
/*t_1_blinding:*/ Scalar,
/*t_2_blinding:*/ Scalar
);

pub fn create_party(
    bp_gens: BulletproofGens,
    pc_gens: PedersenGens,
    v: u64,
    v_blinding: Scalar,
    n: usize) 
    -> CreatePartyRes {
        let mut res = CreatePartyRes::Err(0u8);

        let (party_capacity,gens_capacity,g_vec,h_vec) = bp_gens;

        if !(n == 8 || n == 16 || n == 32 || n == 64) {
            res = CreatePartyRes::Err(INVALID_BIT_SIZE);
        }
        else{if gens_capacity < n {
            res = CreatePartyRes::Err(INVALID_GENERATORS_LENGTH);
        }
        else {
            
            let (base_point,blinding_point) = pc_gens;
            let pedersen_commitment = pedersen_commit(v_blinding, blinding_point, Scalar::from_literal(v as u128), base_point);
            let V = encode(pedersen_commitment);
            res = CreatePartyRes::Ok(((party_capacity,gens_capacity,g_vec,h_vec),pc_gens,n,v,v_blinding,V));
        }}
        res
}

pub fn create_bit_commitment(
    party: PartyAwaitingPosition, 
    j: usize, 
    a_blinding: Scalar, 
    s_blinding: Scalar, 
    s_L: Seq<Scalar>, 
    s_R: Seq<Scalar>) -> AssignPositionRes {

    let mut res = AssignPositionRes::Err(0u8);
    let (bp_gens,pc_gens,n,value,v_blinding,V) = party;
    let (party_capacity, gens_capacity,g_vec,h_vec) = bp_gens;
    let (base_point, blinding_point) = pc_gens;
    if party_capacity <= j {
        res = AssignPositionRes::Err(INVALID_GENERATORS_LENGTH);
    }
    else {

        let v_j = ((value >> j) & 1u64) != 0u64;

        let mut G_j = Seq::<RistrettoPoint>::new(n);
        let mut H_j = Seq::<RistrettoPoint>::new(n);

        let g_vec_j = g_vec[j].clone();
        let h_vec_j = h_vec[j].clone();
    

        for i in 0..n {
            G_j[i] = g_vec_j[i];
            H_j[i] = h_vec_j[i];
        }
        let mut A = IDENTITY_POINT();

        if v_j {
            for i in 0..n { A = add(A,G_j[i]); }
        }
        else {
            for i in 0..n { A = add(A,neg(H_j[i])); }
        }

        let mut S = mul(s_blinding, blinding_point);

        for i in 0..n {
            S = add(S,mul(s_L[i],G_j[i]));
            S = add(S,mul(s_R[i],H_j[i]));
        }
        let new_party = (n,value,v_blinding,j,pc_gens,a_blinding,s_blinding,s_L,s_R);

        res = AssignPositionRes::Ok((new_party,(V,A,S)));
    }
    res
}

pub fn create_poly_commitment(party: PartyAwaitingBitChallenge, bit_challenge: (Scalar,Scalar), t1_blinding: Scalar, t2_blinding: Scalar) -> CreatePolyCommitmentRes {

    let mut res = CreatePolyCommitmentRes::Err(0u8);

    let (n, v, v_blinding, j, pc_gens, a_blinding, s_blinding, s_L, s_R) = party;
    let (base_point, blinding_point) = pc_gens;
    let (y, z) = bit_challenge;


    let offset_y = y^(Scalar::from_literal((j*n) as u128));
    let offset_z = z^Scalar::from_literal(j as u128);

    // Calculate t by calculating vectors l0, l1, r0, r1 and multiplying
    let mut l_poly0 = Seq::<Scalar>::new(n);
    let mut l_poly1 = Seq::<Scalar>::new(n);
    let mut r_poly0 = Seq::<Scalar>::new(n);
    let mut r_poly1 = Seq::<Scalar>::new(n);

    let offset_zz = z^Scalar::from_literal(2u128) * offset_z;
    let mut exp_y = offset_y; // start at y^j
    let mut exp_2 = Scalar::from_literal(1u128); // start at 2^0 = 1
    
    for i in 0..n {
        let a_L_i = Scalar::from_literal(((v >> i) & 1u64) as u128);
        let a_R_i = a_L_i - Scalar::from_literal(1u128);

        l_poly0[i] = a_L_i - z;
        l_poly1[i] = s_L[i];
        r_poly0[i] = exp_y * (a_R_i + z) + offset_zz * exp_2;
        r_poly1[i] = exp_y * s_R[i];

        exp_y = exp_y * y; // y^i -> y^(i+1)
        exp_2 = exp_2 + exp_2; // 2^i -> 2^(i+1)
    }
    
    /* Start of innerproduct method: */
    let mut t0 = Scalar::from_literal(0u128);
    let mut t2 = Scalar::from_literal(0u128);
    let mut l0_plus_l1 = Seq::<Scalar>::new(n);
    let mut r0_plus_r1 = Seq::<Scalar>::new(n);
    let mut t1_temp = Scalar::from_literal(0u128);

    for i in 0..n {
        t0 = t0 + l_poly0[i] * r_poly0[i];
        t2 = t2 + l_poly1[i] * r_poly1[i];
        l0_plus_l1[i] = l_poly0[i] + l_poly1[i];
        r0_plus_r1[i] = r_poly0[i] + r_poly1[i];
        t1_temp = t1_temp + l0_plus_l1[i] * r0_plus_r1[i];
    }
    
    let t1 = t1_temp - t0 - t2;

    let t_poly = (t0,t1,t2);
    /* End of Inner Product method */
    
    // Generate x by committing to T_1, T_2 (line 49-54)
    let T_1 = pedersen_commit(t1_blinding, blinding_point, t1, base_point);
    let T_2 = pedersen_commit(t2_blinding, blinding_point, t2, base_point);
    
    
    let poly_commitment = (T_1, T_2);

    let party_awaiting_poly_challenge = (
        offset_zz,
        (l_poly0, l_poly1),
        (r_poly0, r_poly1),
        t_poly,
        v_blinding,
        a_blinding,
        s_blinding,
        t1_blinding,
        t2_blinding
    );

    res = CreatePolyCommitmentRes::Ok((party_awaiting_poly_challenge, poly_commitment));

    res
}

pub fn create_proofshare(party: PartyAwaitingPolyChallenge, challenge: Scalar) -> CreateProofShareRes {
    let mut res = CreateProofShareRes::Err(0u8);
    if challenge == Scalar::from_literal(0u128) {
        res = CreateProofShareRes::Err(MALICIOUS_DEALER);
    }
    else { 
        let (offset_zz,l_poly,r_poly,t_poly,v_blinding,a_blinding,s_blinding,t_1_blinding,t_2_blinding) = party;

        let (t_poly0, t_poly1, t_poly2) = t_poly;

        let t_x = t_poly0 + challenge * (t_poly1 + challenge * t_poly2);

        let t_x_blinding = (offset_zz * v_blinding) + challenge * (t_1_blinding + challenge * t_2_blinding);
        
        let e_blinding = a_blinding + s_blinding * challenge;

        let (l_poly0, l_poly1) = l_poly;
        let (r_poly0, r_poly1) = r_poly;

        let n = l_poly0.len(); /* l_poly and r_poly are all the same length */

        let mut l_vec = Seq::<Scalar>::new(n);
        let mut r_vec = Seq::<Scalar>::new(n);

        for i in 0..n {
            l_vec[i] = l_poly0[i] + l_poly1[i] * challenge;
            r_vec[i] = r_poly0[i] + r_poly1[i] * challenge;
        }
        let proof_share = (t_x_blinding,t_x, e_blinding, l_vec, r_vec);

        res = CreateProofShareRes::Ok(proof_share);
    }
    res
}