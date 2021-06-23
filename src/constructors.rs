use std::convert::Infallible;

use crate::{Bit, Component, Gate, GateKind, GateLike, RustImpls, Sequal};

pub struct ComponentBuilder<Rust = Infallible> {
    gates: Vec<Gate<Rust>>,
    sequals: Vec<Vec<Sequal>>,
    outputs: usize,
}

impl<Rust> Default for ComponentBuilder<Rust> {
    fn default() -> Self {
        Self {
            gates: Vec::default(),
            sequals: Vec::default(),
            outputs: usize::default(),
        }
    }
}

impl<Rust> ComponentBuilder<Rust> {
    pub fn gate(mut self, gate: Gate<Rust>, sequals: Vec<Sequal>) -> Self {
        self.push_sequals(sequals);
        self.gates.push(gate);
        self
    }

    fn push_sequals(&mut self, sequals: Vec<Sequal>) -> &mut Self {
        self.outputs += sequals
            .iter()
            .filter(|x| matches!(x, Sequal::End { .. }))
            .count();
        self.sequals.push(sequals);
        self
    }

    pub fn inputs(mut self, inputs: Vec<Sequal>) -> Component<Rust> {
        self.push_sequals(inputs);
        Component::from_raw_parts(self.gates, self.sequals, self.outputs)
    }
}

impl<Rust> Component<Rust> {
    pub fn builder() -> ComponentBuilder<Rust> {
        ComponentBuilder::default()
    }

    pub fn from_raw_parts(
        gates: Vec<Gate<Rust>>,
        sequals: Vec<Vec<Sequal>>,
        outputs: usize,
    ) -> Self {
        debug_assert_eq!(gates.len() + 1, sequals.len());

        Self {
            gates,
            sequals,
            outputs,
        }
    }
}

impl Sequal {
    pub fn gate(index: usize, entry: usize) -> Self {
        Self::Gate { index, entry }
    }

    pub fn end(output: usize) -> Self {
        Self::End { output }
    }
}

impl<Rust> Gate<Rust>
where
    Rust: GateLike,
{
    pub fn new(kind: GateKind<Rust>) -> Self {
        Self {
            inputs: zeroed_vec(kind.num_of_inputs()),
            inputs_filled: 0,
            kind,
        }
    }

    pub fn custom_curcuit(component: Component<Rust>) -> Self {
        Self::new(GateKind::Custom(component))
    }
    pub fn rust_logic(logic: Rust) -> Self {
        Self::new(GateKind::Rust(RustImpls::User(logic)))
    }
    pub fn dup(amount: usize) -> Self {
        Self::new(GateKind::Rust(RustImpls::Dup(amount)))
    }
    pub fn not() -> Self {
        Self::new(GateKind::Rust(RustImpls::Not))
    }
    pub fn nand() -> Self {
        Self::new(GateKind::Rust(RustImpls::Nand))
    }
    pub fn and() -> Self {
        Self::new(GateKind::Rust(RustImpls::And))
    }
    pub fn or() -> Self {
        Self::new(GateKind::Rust(RustImpls::Or))
    }
    pub fn nor() -> Self {
        Self::new(GateKind::Rust(RustImpls::Nor))
    }
    pub fn xor() -> Self {
        Self::new(GateKind::Rust(RustImpls::Xor))
    }
}

pub(crate) fn zeroed_vec(len: usize) -> Vec<Bit> {
    let mut v = Vec::new();
    v.resize_with(len, Bit::default);
    v
}
