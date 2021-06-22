use crate::{Bit, Gate, Sequal};

pub(super) trait SequalsExtension {
    fn run(
        &self,
        for_gate: usize,
        output: Vec<Bit>,
        gates: &mut [Gate],
        outputs: &mut Vec<Bit>,
        outputs_filled: &mut usize,
    );
}

impl SequalsExtension for [Vec<Sequal>] {
    fn run(
        &self,
        for_gate: usize,
        output: Vec<Bit>,
        gates: &mut [Gate],
        outputs: &mut Vec<Bit>,
        outputs_filled: &mut usize,
    ) {
        let sequals = unsafe { self.get_unchecked(for_gate) };
        debug_assert_eq!(sequals.len(), output.len(), "index is {}", for_gate);

        for (sequal, data) in sequals.iter().copied().zip(output.into_iter()) {
            match sequal {
                Sequal::End { output } => {
                    *outputs_filled += 1;
                    *unsafe { outputs.get_unchecked_mut(output) } = data;
                }
                Sequal::Gate { index, entry } => {
                    if let Some(output) =
                        unsafe { gates.get_unchecked_mut(index) }.fill_input(data, entry)
                    {
                        self.run(index, output, gates, outputs, outputs_filled);
                    }
                }
            }
        }
    }
}
