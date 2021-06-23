use std::mem;

use crate::{constructors, Bit, Gate, GateKind, GateLike};

impl<Rust> Gate<Rust>
where
    Rust: GateLike,
{
    pub(super) fn fill_input(&mut self, with: Bit, at: usize) -> Option<Vec<Bit>> {
        debug_assert!(
            self.inputs.len() > at,
            "tried to insert at [{}], but length was {}",
            at,
            self.inputs.len()
        );
        *unsafe { self.inputs.get_unchecked_mut(at) } = with;
        self.inputs_filled += 1;
        if self.inputs_filled == self.inputs.len() {
            Some(self.compute())
        } else {
            None
        }
    }

    fn compute(&mut self) -> Vec<Bit> {
        let inputs = mem::take(&mut self.inputs);

        self.kind.compute(inputs)
    }

    pub(super) fn reset(&mut self) {
        self.inputs_filled = 0;
        self.inputs = constructors::zeroed_vec(self.kind.num_of_inputs());
        if let GateKind::Custom(inner) = &mut self.kind {
            inner.reset();
        }
    }
}

impl<Rust> GateLike for GateKind<Rust>
where
    Rust: GateLike,
{
    fn compute(&mut self, input: Vec<Bit>) -> Vec<Bit> {
        match self {
            GateKind::Custom(component) => component.compute(input),
            GateKind::Rust(component) => component.compute(input),
        }
    }

    fn num_of_inputs(&self) -> usize {
        match self {
            Self::Custom(component) => component.num_of_inputs(),
            Self::Rust(component) => component.num_of_inputs(),
        }
    }
}
