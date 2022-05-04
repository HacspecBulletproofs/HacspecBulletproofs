use hacspec_lib::*;

public_nat_mod!(
    type_name: FieldElement,
    type_of_canvas: FieldCanvas,
    bit_size_of_field: 256,
    modulo_value: "00000000000000000000000000000000000000000000000000000000000000ff"
);

fn main() {
    let lhs = FieldElement::from_literal(252);
    let rhs = FieldElement::from_literal(4);
    println!("lhs: {}", lhs);                          //252
    println!("rhs: {}", rhs);                          //4
    println!("rhs.inv(): {}", rhs.inv());              //4
    println!("lhs * rhs.inv(): {}", lhs * rhs.inv());  //243
}
