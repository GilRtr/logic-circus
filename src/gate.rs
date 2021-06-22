use std::mem;

use crate::{constructors, Bit, Gate, GateKind};

impl Gate {
    pub(super) fn fill_input(&mut self, with: Bit, at: usize) -> Option<Vec<Bit>> {
        debug_assert!(
            self.inputs.len() > at,
            "tried to insert at [{}], but length was {}",
            at,
            self.inputs.len()
        );
        *unsafe { self.inputs.get_unchecked_mut(at) } = with;
        self.inputs_filled += 1;
        (self.inputs_filled == self.inputs.len()).then(|| self.compute())
    }

    fn compute(&mut self) -> Vec<Bit> {
        let mut inputs = mem::take(&mut self.inputs);
        match &mut self.kind {
            GateKind::Nand => {
                debug_assert_eq!(inputs.len(), 2);
                inputs.clear();
                inputs.push(!(unsafe { *inputs.get_unchecked(0) && *inputs.get_unchecked(1) }));
                inputs
            }
            GateKind::Duplicate(ref amount) => {
                debug_assert_eq!(inputs.len(), 1);
                inputs.resize(*amount, *unsafe { inputs.get_unchecked(0) });
                inputs
            }
            GateKind::Custom(component) => component.run(inputs),
        }
    }

    pub(super) fn reset(&mut self) {
        self.inputs_filled = 0;
        self.inputs = constructors::zeroed_vec(self.kind.num_of_inputs());
        if let GateKind::Custom(inner) = &mut self.kind {
            inner.reset();
        }
    }
}

impl GateKind {
    pub(super) fn num_of_inputs(&self) -> usize {
        match self {
            Self::Nand => 2,
            Self::Duplicate(_) => 1,
            Self::Custom(component) => {
                unsafe { component.sequals.get_unchecked(component.sequals.len() - 1) }.len()
            }
        }
    }
}
