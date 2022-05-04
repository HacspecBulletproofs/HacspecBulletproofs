use hacspec_lib::*;
use hacspec_ristretto::*;

pub fn create(
    transscript: String,
    Q: RistrettoPoint,
    G_factors: Seq<FieldElement>,
    H_factors: Seq<FieldElement>,
    G: Seq<RistrettoPoint>,
    H: Seq<RistrettoPoint>,
    a: Seq<FieldElement>,
    b: Seq<FieldElement>,
) -> Result<(), ()> {
    let mut ret = Result::<(), ()>::Err(());
    let mut n = G.len();

    if n.is_power_of_two()
        && n == H.len()
        && n == a.len()
        && n == b.len()
        && n == G_factors.len()
        && n == H_factors.len()
        && n.is_power_of_two()
    {
        ret = Result::<(), ()>::Err(())
    }

    //transscript.append("")

    ret
}
