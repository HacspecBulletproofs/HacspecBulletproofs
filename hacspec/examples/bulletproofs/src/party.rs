#![allow(non_snake_case)]
mod transcript;
mod errors;
mod types;
//use transcript::*;
use errors::*;
use types::*;

use hacspec_lib::*;
//use hacspec_merlin::*;
use wrapper_hacspec_ristretto::*;
//use hacspec_ipp::*;
use hacspec_pedersen::*;

type CreatePartyRes = Result<PartyAwaitingPosition,u8>;
type CreateBitCommitmentRes = Result<(RistrettoPointEncoded,RistrettoPoint,RistrettoPoint),u8>;
type CreatePolyCommitmentRes = Result<(PartyAwaitingPolyChallenge,(RistrettoPoint,RistrettoPoint)), u8>;
type CreateProofShareRes = Result<ProofShare,u8>;


#[derive(Clone,Default)]
pub struct PartyAwaitingPosition(
    /*bp_gens:*/ pub BulletproofGens,
    /*pc_gens:*/ pub PedersenGens,
    /*n: usize,*/
    /*v: u64,*/
    /*v_blinding: Scalar,*/
    /*V:*/ pub RistrettoPointEncoded
);

/*pub type PartyAwaitingBitChallenge = (
        /*n: usize,*/ /*bitsize of the range*/
        /*v: u64,*/
        /*v_blinding: Scalar,*/
        /*j: usize,*/
        /*pc_gens: PedersenGens,*/
        /*a_blinding: Scalar,*/
        /*s_blinding: Scalar,*/
        /*s_L: Seq<Scalar>,*/
        /*s_R: Seq<Scalar>*/
);*/ //this is the type used by the rust implementation, but every value in here either unused or too trivial to store

#[derive(Clone,Default)]
pub struct BitChallengeInput(
    /*j:*/ pub usize, 
    /*value:*/ pub u64, 
    /*n:*/ pub usize, 
    /*bit_challenge:*/ pub (Scalar,Scalar), 
    /*pc_gens:*/ pub PedersenGens, 
    /*t1_blinding:*/ pub Scalar, 
    /*t2_blinding:*/ pub Scalar, 
    /*s_L:*/ pub Seq<Scalar>, 
    /*s_R:*/ pub Seq<Scalar>
);

#[derive(Clone,Default)]
pub struct PartyAwaitingPolyChallenge(
/*offset_zz:*/ pub Scalar,
/*l_poly:*/ pub (Seq<Scalar>, Seq<Scalar>),
/*r_poly:*/ pub (Seq<Scalar>, Seq<Scalar>),
/*t_poly:*/ pub (Scalar, Scalar, Scalar),
/*v_blinding: Scalar,*/
/*a_blinding: Scalar,*/
/*s_blinding: Scalar,*/
/*t_1_blinding: Scalar,*/
/*t_2_blinding: Scalar*/
);

/* HELPER METHODS */

fn inner_product(left: Seq<Scalar>, right: Seq<Scalar>) -> Scalar {
    let mut res = Scalar::from_literal(0u128);

    for i in 0..left.len() {
        res = res + (left[i] * right[i]);
    }
    res
}

fn scalar_exp(mut result: Scalar, x: Scalar, exp: u64) -> Scalar{
    let mut ret = result;
    let mut aux = x; // x, x^2, x^4, x^8, ...
    let mut n = exp;

    if n > 0 {
        let bit = n & 1;
        if bit == 1 {
            result = result * aux;
            n = n >> 1;
            aux = aux * aux;
        }
        else {
        n = n >> 1;
        aux = aux * aux; // FIXME: one unnecessary mult at the last step here!
        }
        ret = scalar_exp(result, aux, n);
        
    }
    ret
}

pub fn create_party(
    bp_gens: BulletproofGens,
    pc_gens: PedersenGens,
    v: u64,
    v_blinding: Scalar,
    n: usize) 
    -> CreatePartyRes {

        #[allow(unused)]
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
            res = CreatePartyRes::Ok(PartyAwaitingPosition((party_capacity,gens_capacity,g_vec,h_vec),pc_gens,V));
        }}
        res
}

pub fn create_bit_commitment(
    party: PartyAwaitingPosition, 
    value: u64,
    n: usize,
    j: usize, 
    a_blinding: Scalar, 
    s_blinding: Scalar, 
    s_L: Seq<Scalar>, 
    s_R: Seq<Scalar>) -> CreateBitCommitmentRes {

    #[allow(unused)]
    let mut res = CreateBitCommitmentRes::Err(0u8);

    let PartyAwaitingPosition(bp_gens,pc_gens,V) = party;

    let (party_capacity, _,g_vec,h_vec) = bp_gens;

    let (_, blinding_point) = pc_gens;

    if party_capacity <= j {
        res = CreateBitCommitmentRes::Err(INVALID_GENERATORS_LENGTH);
    }
    else {

        let mut A = mul(a_blinding, blinding_point);


        let mut G_i = Seq::<RistrettoPoint>::new(n);
        let mut H_i = Seq::<RistrettoPoint>::new(n);
        

        let g_vec_j = g_vec[j].clone();
        let h_vec_j = h_vec[j].clone();
    

        for i in 0..n {
            G_i[i] = g_vec_j[i];
            H_i[i] = h_vec_j[i];
        }

        for i in 0..n {
            let v_i = ((value >> i) & 1u64) != 0u64;
            if v_i {
                A = add(A,G_i[i]);
            }
            else {
                A = add(A,neg(H_i[i])); 
            }
        }


        let mut S = mul(s_blinding, blinding_point);


        for i in 0..n {
            S = add(S,mul(s_L[i],G_i[i]));
            S = add(S,mul(s_R[i],H_i[i]));
        }

        res = CreateBitCommitmentRes::Ok((V,A,S));
    }
    res
}

pub fn create_poly_commitment(party: BitChallengeInput) -> CreatePolyCommitmentRes {

    #[allow(unused)]
    let mut res = CreatePolyCommitmentRes::Err(0u8);

    let BitChallengeInput(j, value, n, bit_challenge, pc_gens, t1_blinding, t2_blinding, s_L, s_R) = party;

    let (base_point, blinding_point) = pc_gens;
    let (y, z) = bit_challenge;

    let offset_y = scalar_exp(Scalar::from_literal(1u128), y,(j*n) as u64);
    let offset_z = scalar_exp(Scalar::from_literal(1u128), z, j as u64);

    // Calculate t by calculating vectors l0, l1, r0, r1 and multiplying
    let mut l_poly0 = Seq::<Scalar>::new(n);
    let mut l_poly1 = Seq::<Scalar>::new(n);
    let mut r_poly0 = Seq::<Scalar>::new(n);
    let mut r_poly1 = Seq::<Scalar>::new(n);

    let offset_zz = z * z * offset_z;
    let mut exp_y = offset_y; // start at y^j
    let mut exp_2 = Scalar::from_literal(1u128); // start at 2^0 = 1
    
    for i in 0..n {
        let a_L_i = Scalar::from_literal(((value >> i) & 1u64) as u128);
        let a_R_i = a_L_i - Scalar::from_literal(1u128);

        l_poly0[i] = a_L_i - z;
        l_poly1[i] = s_L[i];

        r_poly0[i] = exp_y * (a_R_i + z) + offset_zz * exp_2;
        r_poly1[i] = exp_y * s_R[i];
        
        exp_y = exp_y * y; // y^i -> y^(i+1)
        exp_2 = exp_2 + exp_2; // 2^i -> 2^(i+1)
    }
    
    let t0 = inner_product(l_poly0.clone(),r_poly0.clone());

    let t2 = inner_product(l_poly1.clone(),r_poly1.clone());
    let mut l0_plus_l1 = Seq::<Scalar>::new(n);
    let mut r0_plus_r1 = Seq::<Scalar>::new(n);

    for i in 0..n {
        l0_plus_l1[i] = l_poly0[i] + l_poly1[i];
        r0_plus_r1[i] = r_poly0[i] + r_poly1[i];
    }
    
    let t1 = inner_product(l0_plus_l1,r0_plus_r1) - t0 - t2;

    let t_poly = (t0,t1,t2);
    /* End of Inner Product method */
    
    // Generate x by committing to T_1, T_2 (line 49-54)
    let T_1 = pedersen_commit(t1_blinding, blinding_point, t1, base_point);
    let T_2 = pedersen_commit(t2_blinding, blinding_point, t2, base_point);
    
    
    let poly_commitment = (T_1, T_2);

    let party_awaiting_poly_challenge = PartyAwaitingPolyChallenge(
        offset_zz,
        (l_poly0, l_poly1),
        (r_poly0, r_poly1),
        t_poly,
    );

    res = CreatePolyCommitmentRes::Ok((party_awaiting_poly_challenge, poly_commitment));

    res
}

pub fn create_proofshare(party: PartyAwaitingPolyChallenge, 
                         v_blinding: Scalar, 
                         a_blinding: Scalar, 
                         s_blinding: Scalar, 
                         t_1_blinding:Scalar, 
                         t_2_blinding: Scalar, 
                         challenge: Scalar) 
                         -> CreateProofShareRes {

    #[allow(unused)]
    let mut res = CreateProofShareRes::Err(0u8);

    if challenge == Scalar::from_literal(0u128) {
        res = CreateProofShareRes::Err(MALICIOUS_DEALER);
    }

    else { 
        let PartyAwaitingPolyChallenge(offset_zz,l_poly,r_poly,t_poly) = party;

        let (t_poly0, t_poly1, t_poly2) = t_poly;

        let t_x = t_poly0 + (challenge * (t_poly1 + (challenge * t_poly2)));

        
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

        let proof_share = (t_x,t_x_blinding, e_blinding, l_vec, r_vec);

        res = CreateProofShareRes::Ok(proof_share);
    }
    res
}