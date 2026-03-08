#![allow(dead_code)]

use proptest::prelude::*;
use symbolic_mr::{
    Index, OperatorProduct, active, annihilate, core, create, general, operator_string, virtual_,
};

pub fn one_body_supported_strategy() -> BoxedStrategy<OperatorProduct> {
    prop_oneof![
        (core_index_strategy(), core_index_strategy())
            .prop_map(|(left, right)| create(left) * annihilate(right)),
        (active_index_strategy(), active_index_strategy())
            .prop_map(|(left, right)| create(left) * annihilate(right)),
    ]
    .boxed()
}

pub fn two_body_active_only_strategy() -> BoxedStrategy<OperatorProduct> {
    (
        active_index_strategy(),
        active_index_strategy(),
        active_index_strategy(),
        active_index_strategy(),
    )
        .prop_map(|(u, v, w, x)| {
            operator_string(vec![create(u), create(v), annihilate(w), annihilate(x)])
        })
        .boxed()
}

#[allow(dead_code)]
pub fn two_body_core_active_mixed_strategy() -> BoxedStrategy<OperatorProduct> {
    (
        core_index_strategy(),
        active_index_strategy(),
        core_index_strategy(),
        active_index_strategy(),
    )
        .prop_map(|(i, u, j, v)| {
            operator_string(vec![create(i), create(u), annihilate(j), annihilate(v)])
        })
        .boxed()
}

#[allow(dead_code)]
pub fn virtual_containing_zero_strategy() -> BoxedStrategy<OperatorProduct> {
    prop_oneof![
        (virtual_index_strategy(), virtual_index_strategy())
            .prop_map(|(a, b)| create(a) * annihilate(b)),
        (
            virtual_index_strategy(),
            active_index_strategy(),
            active_index_strategy(),
            virtual_index_strategy(),
        )
            .prop_map(|(a, u, v, b)| {
                operator_string(vec![create(a), create(u), annihilate(v), annihilate(b)])
            }),
        Just(operator_string(vec![
            create(active("u")),
            create(active("v")),
            create(virtual_("a")),
            annihilate(virtual_("b")),
            annihilate(active("v")),
            annihilate(active("u")),
        ])),
    ]
    .boxed()
}

pub fn repeated_index_cases_strategy() -> BoxedStrategy<OperatorProduct> {
    prop_oneof![
        Just(operator_string(vec![
            create(active("u")),
            create(active("u")),
            annihilate(active("v")),
            annihilate(active("v")),
        ])),
        Just(create(core("i")) * annihilate(core("i"))),
        Just(operator_string(vec![
            create(general("p")),
            annihilate(general("p")),
            create(general("q")),
            annihilate(general("q")),
        ])),
    ]
    .boxed()
}

pub fn non_normal_ordered_cases_strategy() -> BoxedStrategy<OperatorProduct> {
    prop_oneof![
        (active_index_strategy(), active_index_strategy()).prop_map(|(left, right)| annihilate(
            left
        ) * create(
            right
        )),
        (
            active_index_strategy(),
            active_index_strategy(),
            active_index_strategy(),
            active_index_strategy(),
        )
            .prop_map(|(x, u, v, w)| {
                operator_string(vec![annihilate(x), create(u), create(v), annihilate(w)])
            }),
    ]
    .boxed()
}

pub fn unsupported_general_index_cases_strategy() -> BoxedStrategy<OperatorProduct> {
    prop_oneof![
        Just(create(general("p")) * annihilate(general("p"))),
        Just(create(general("p")) * annihilate(general("q"))),
        Just(operator_string(vec![
            create(general("p")),
            create(general("q")),
            annihilate(active("u")),
            annihilate(general("r")),
        ])),
        Just(operator_string(vec![
            create(general("p")),
            create(general("q")),
            annihilate(general("r")),
            annihilate(general("s")),
        ])),
    ]
    .boxed()
}

#[allow(dead_code)]
pub fn unsupported_higher_body_cases_strategy() -> BoxedStrategy<OperatorProduct> {
    Just(operator_string(vec![
        create(core("i")),
        create(active("u")),
        create(active("v")),
        annihilate(active("v")),
        annihilate(active("u")),
        annihilate(core("j")),
    ]))
    .boxed()
}

fn core_index_strategy() -> BoxedStrategy<Index> {
    prop_oneof![Just(core("i")), Just(core("j"))].boxed()
}

fn active_index_strategy() -> BoxedStrategy<Index> {
    prop_oneof![Just(active("u")), Just(active("v"))].boxed()
}

fn virtual_index_strategy() -> BoxedStrategy<Index> {
    prop_oneof![Just(virtual_("a")), Just(virtual_("b"))].boxed()
}

fn general_index_strategy() -> BoxedStrategy<Index> {
    prop_oneof![
        Just(general("p")),
        Just(general("q")),
        Just(general("r")),
        Just(general("s")),
    ]
    .boxed()
}
