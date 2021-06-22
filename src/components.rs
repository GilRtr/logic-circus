use std::collections::HashMap;

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
        assert_eq!(component.run(input), output);
        component.reset();
    }
}

#[test]
fn todo() {
    panic!("comp doesn't work for more than 2 outputs / inputs");
}

#[test]
fn nand_test() {
    let nand = comp! {
        (0 => 0, 0 => 1);
        nand: 0;
    };

    check(
        nand,
        table! {
            false, false => true;
            false, true => true;
            true, false => true;
            true, true => false;
        },
    )
}

#[test]
fn dup_test() {
    let dup = comp! {
        0 => 0;
        dup: (0, 1);
    };

    check(
        dup,
        table! {
            false => false, false;
            true => true, true;
        },
    );
}

fn not() -> Component {
    comp! {
        0 => 0;
        dup: (1 => 0, 1 => 1);
        nand: 0;
    }
}

#[test]
fn not_test() {
    check(
        not(),
        table! {
            true => false;
            false => true;
        },
    );
}

fn and() -> Component {
    comp! {
        (0 => 0, 0 => 1);
        nand: 1 => 0;
        not: 0;
    }
}

#[test]
fn and_test() {
    check(
        and(),
        table! {
            false, false => false;
            false, true => false;
            true, false => false;
            true, true => true;
        },
    );
}

fn or() -> Component {
    comp! {
        (0 => 0, 1 => 0);
        not: 2 => 0;
        not: 2 => 1;
        nand: 0;
    }
}

#[test]
fn or_test() {
    check(
        or(),
        table! {
            false, false => false;
            false, true => true;
            true, false => true;
            true, true => true;
        },
    );
}

fn nor() -> Component {
    comp! {
        (0 => 0, 0 => 1);
        or: 1 => 0;
        not: 0;
    }
}

#[test]
fn nor_test() {
    check(
        nor(),
        table! {
            false, false => true;
            false, true => false;
            true, false => false;
            true, true => false;
        },
    );
}

fn xor() -> Component {
    comp! {
        (0 => 0, 1 => 0);
        dup: (2 => 0, 4 => 0);
        dup: (2 => 1, 4 => 1);
        and: 3 => 0;
        not: 5 => 1;
        or: 5 => 0;
        and: 0;
    }
}

#[test]
fn xor_test() {
    check(
        xor(),
        table! {
            false, false => false;
            false, true => true;
            true, false => true;
            true, true => false;
        },
    );
}
