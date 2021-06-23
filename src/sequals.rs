use crate::{Bit, Gate, GateLike, Sequal};

pub(super) trait SequalsExtension<Rust> {
    fn run(
        &self,
        for_gate: usize,
        output: &[Bit],
        gates: &mut [Gate<Rust>],
        outputs: &mut Vec<Bit>,
    );
}

impl<Rust> SequalsExtension<Rust> for [Vec<Sequal>]
where
    Rust: GateLike,
{
    fn run(
        &self,
        for_gate: usize,
        output: &[Bit],
        gates: &mut [Gate<Rust>],
        outputs: &mut Vec<Bit>,
    ) {
        debug_assert!(for_gate < self.len());
        let sequals = unsafe { self.get_unchecked(for_gate) };

        debug_assert_eq!(sequals.len(), output.len(), "index is {}", for_gate);
        for (sequal, data) in sequals.iter().copied().zip(output.iter().copied()) {
            match sequal {
                Sequal::End { output } => {
                    *unsafe { outputs.get_unchecked_mut(output) } = data;
                }
                Sequal::Gate { index, entry } => {
                    if let Some(output) =
                        unsafe { gates.get_unchecked_mut(index) }.fill_input(data, entry)
                    {
                        self.run(index, &output, gates, outputs);
                        drop(output); // just want to emphasize this is where deallocation happens for Gate::inputs
                    }
                }
            }
        }
    }
}
