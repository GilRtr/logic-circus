use crate::{Bit, Component, Gate, GateKind, Sequal};

impl Component {
    pub fn new(
        gates: Vec<Gate>,
        input_sequals: Vec<Sequal>,
        mut sequals: Vec<Vec<Sequal>>,
    ) -> Self {
        debug_assert_eq!(gates.len(), sequals.len());
        sequals.push(input_sequals);
        Self {
            outputs: sequals
                .iter()
                .flatten()
                .filter(|x| matches!(x, Sequal::End { .. }))
                .count(),
            gates,
            sequals,
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

impl Gate {
    pub fn new(kind: GateKind) -> Self {
        Self {
            inputs: zeroed_vec(kind.num_of_inputs()),
            inputs_filled: 0,
            kind,
        }
    }

    pub fn nand_gate() -> Self {
        Self::new(GateKind::Nand)
    }
    pub fn duplicator(amount: usize) -> Self {
        Self::new(GateKind::Duplicate(amount))
    }
    pub fn custom(component: Component) -> Self {
        Self::new(GateKind::Custom(component))
    }
}

pub(crate) fn zeroed_vec(len: usize) -> Vec<Bit> {
    let mut v = Vec::new();
    v.resize_with(len, Bit::default);
    v
}
