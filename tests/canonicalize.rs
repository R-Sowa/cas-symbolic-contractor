use symbolic_mr::{active, annihilate, create, operator_string};

#[test]
fn canonicalizes_operator_order_deterministically() {
    let u = active("u");
    let v = active("v");
    let ops = operator_string(vec![annihilate(v), create(u)]);
    let canonical = ops.canonicalize();
    assert_eq!(canonical.to_string(), "a†(u) a(v)");
}
