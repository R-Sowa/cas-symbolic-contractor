use symbolic_mr::{core, delta, simplify_tensor_form, virtual_};

#[test]
fn contradictory_space_equalities_reduce_to_zero() {
    let i = core("i");
    let a = virtual_("a");

    let expr = delta(i, a);
    let simplified = simplify_tensor_form(expr, Default::default()).unwrap();

    assert_eq!(simplified.to_string(), "0");
}
