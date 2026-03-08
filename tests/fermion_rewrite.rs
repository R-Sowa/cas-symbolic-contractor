use symbolic_mr::{annihilate, create, general, simplify_operator_form};

#[test]
fn swaps_adjacent_fermions_with_sign_and_delta() {
    let p = general("p");
    let q = general("q");
    let expr = annihilate(p.clone()) * create(q.clone());
    let simplified = simplify_operator_form(expr, Default::default()).unwrap();
    assert_eq!(simplified.to_string(), "delta(p,q) - a†(q) a(p)");
}
