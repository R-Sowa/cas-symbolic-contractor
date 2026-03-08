use symbolic_mr::{annihilate, create, general, normal_order_product};

#[test]
fn normal_orders_adjacent_annihilate_create_pair() {
    let p = general("p");
    let q = general("q");

    let result = normal_order_product(annihilate(p.clone()) * create(q.clone())).unwrap();
    assert_eq!(result.to_string(), "delta(p,q) - a†(q) a(p)");
}

#[test]
fn swaps_same_kind_creators_with_minus_sign() {
    let u = general("u");
    let v = general("v");

    let result = normal_order_product(create(v) * create(u)).unwrap();
    assert_eq!(result.to_string(), "- a†(u) a†(v)");
}

#[test]
fn swaps_same_kind_annihilators_with_minus_sign() {
    let u = general("u");
    let v = general("v");

    let result = normal_order_product(annihilate(v) * annihilate(u)).unwrap();
    assert_eq!(result.to_string(), "- a(u) a(v)");
}

#[test]
fn normal_orders_four_operator_string_with_delta_branch() {
    let p = general("p");
    let q = general("q");
    let r = general("r");
    let s = general("s");

    let expr =
        annihilate(p.clone()) * create(q.clone()) * create(r.clone()) * annihilate(s.clone());
    let result = normal_order_product(expr).unwrap();
    let rendered = result.to_string();

    assert!(rendered.contains("delta(p,q)"));
    assert!(rendered.contains("a†(r) a(s)"));
}

#[test]
fn normal_orders_double_crossing_case_without_losing_delta_terms() {
    let p = general("p");
    let q = general("q");
    let r = general("r");
    let s = general("s");

    let expr =
        annihilate(p.clone()) * annihilate(q.clone()) * create(r.clone()) * create(s.clone());
    let result = normal_order_product(expr).unwrap();
    let rendered = result.to_string();

    assert!(rendered.contains("delta"));
}
