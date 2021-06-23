use std::collections::HashMap;

use crate::{Component, Gate, GateLike, Sequal};

use super::*;

macro_rules! table {
    ($($($key:expr),* $(,)? => $($value:expr),* $(,)? );* $(;)?) => {{
        let mut map = HashMap::new();
        $(map.insert(vec![$($key),*], vec![$($value),*]);)*
        map
    }};
}

fn check(mut component: Component, truth_table: HashMap<Vec<Bit>, Vec<Bit>>) {
    for (input, output) in truth_table {
        assert_eq!(component.compute(input), output);
        component.reset();
    }
}

fn component(gate: Gate) -> Component {
    let sequals = vec![Sequal::end(0)];
    let inputs = (0..gate.kind.num_of_inputs())
        .map(|x| Sequal::gate(0, x))
        .collect();
    Component::builder().gate(gate, sequals).inputs(inputs)
}

#[test]
fn nand() {
    check(
        component(Gate::nand()),
        table! {
                false, false => true;
                false, true => true;
                true, false => true;
                true, true => false;
        },
    )
}

#[test]
fn dup() {
    check(
        Component::builder()
            .gate(Gate::dup(3), (0..3).map(Sequal::end).collect())
            .inputs(vec![Sequal::gate(0, 0)]),
        table! {
            false => false, false, false;
            true => true, true, true;
        },
    )
}

#[test]
fn not() {
    check(
        component(Gate::not()),
        table! {
            true => false;
            false => true;
        },
    )
}

#[test]
fn and() {
    check(
        component(Gate::and()),
        table! {
            false, false => false;
            false, true => false;
            true, false => false;
            true, true => true;
        },
    )
}

#[test]
fn or() {
    check(
        component(Gate::or()),
        table! {
            false, false => false;
            false, true => true;
            true, false => true;
            true, true => true;
        },
    )
}

#[test]
fn nor() {
    check(
        component(Gate::nor()),
        table! {
            false, false => true;
            false, true => false;
            true, false => false;
            true, true => false;
        },
    )
}

#[test]
fn xor() {
    check(
        component(Gate::xor()),
        table! {
            false, false => false;
            false, true => true;
            true, false => true;
            true, true => false;
        },
    )
}
