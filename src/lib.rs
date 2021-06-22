use sequals::SequalsExtension;

#[cfg(test)]
mod components;
mod constructors;
mod gate;
mod macros;
mod sequals;

type Bit = bool;

#[cfg(test)]
mod todo {
    #[test]
    fn comp_macro() {
        panic!("The comp macro doesn't work for more than 2 outputs / inputs");
    }

    #[test]
    fn optimize() {
        panic!("Switch out the bit vectors to SmallBitVec. Provide built-in implementations for gates that are common and would greatly benifit from it");
    }

    #[test]
    fn memory() {
        panic!("Provide a way to save things in registers / ram");
    }

    #[test]
    fn expand() {
        panic!("Provide infrastructure for multibit operations (multibit input & output and a primitive split gate)");
    }
}

#[derive(Clone, Debug)]
pub struct Component {
    gates: Vec<Gate>,
    /// for every output in each gate (`sequals[0][1]` is for the second output of the first gate)
    /// sequals.last
    sequals: Vec<Vec<Sequal>>,
    /// number of outputs
    outputs: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum Sequal {
    Gate { index: usize, entry: usize },
    End { output: usize },
}

#[derive(Clone, Debug)]
pub struct Gate {
    kind: GateKind,
    inputs: Vec<Bit>,
    inputs_filled: usize,
}

#[derive(Clone, Debug)]
pub enum GateKind {
    Nand,
    Duplicate(usize),
    Custom(Component),
    // Split,
    // Clock,
}

impl Component {
    pub fn run(&mut self, input: Vec<Bit>) -> Vec<Bit> {
        let mut outputs = constructors::zeroed_vec(self.outputs);
        let mut outputs_filled = 0;

        let gates = &mut self.gates;

        self.sequals.run(
            self.sequals.len() - 1,
            input,
            gates,
            &mut outputs,
            &mut outputs_filled,
        );

        debug_assert_eq!(outputs_filled, outputs.len());
        outputs
    }

    pub fn reset(&mut self) {
        for gate in &mut self.gates {
            gate.reset()
        }
    }
}
