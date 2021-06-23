#![warn(clippy::cargo_common_metadata)]
#![allow(unused_unsafe)]

//! Logic Circus is a logic circut simulator written in Rust
//!
//! It is nowhere near finished and I'd like to one day even add a GUI
//! You can build circuits by composing other circuits together (with
//! `Component`) or by fully implementing their logic (with the `GateLike`
//! trait)

use std::convert::Infallible;

use sequals::SequalsExtension;

mod constructors;
mod gate;
mod implemented;
mod sequals;
#[cfg(test)]
mod todo;
mod util;

type Bit = bool;

#[derive(Clone, Debug)]
pub struct Component<Rust = Infallible> {
    gates: Vec<Gate<Rust>>,
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
pub struct Gate<Rust = Infallible> {
    kind: GateKind<Rust>,
    inputs: Vec<Bit>,
    inputs_filled: usize,
}

#[derive(Clone, Debug)]
pub enum GateKind<Rust = Infallible> {
    Custom(Component<Rust>),
    Rust(RustImpls<Rust>),
}

#[derive(Clone, Debug, Copy)]
pub enum RustImpls<Rust = Infallible> {
    Dup(usize),
    Not,
    Nand,
    And,
    Or,
    Nor,
    Xor,
    User(Rust),
}

pub trait GateLike {
    fn compute(&mut self, input: Vec<Bit>) -> Vec<Bit>;
    fn num_of_inputs(&self) -> usize;
}

impl<Rust> Component<Rust>
where
    Rust: GateLike,
{
    pub fn reset(&mut self) {
        for gate in &mut self.gates {
            gate.reset()
        }
    }
}

impl<Rust> GateLike for Component<Rust>
where
    Rust: GateLike,
{
    fn compute(&mut self, input: Vec<Bit>) -> Vec<Bit> {
        let mut outputs = constructors::zeroed_vec(self.outputs);

        self.sequals.run(
            self.sequals.len() - 1,
            &input,
            &mut self.gates,
            &mut outputs,
        );

        outputs
    }

    fn num_of_inputs(&self) -> usize {
        unsafe { self.sequals.get_unchecked(self.sequals.len() - 1) }.len()
    }
}

impl GateLike for Infallible {
    fn compute(&mut self, _input: Vec<Bit>) -> Vec<Bit> {
        unreachable!("can't compute result of infallible")
    }

    fn num_of_inputs(&self) -> usize {
        unreachable!("infallible has <???> inputs")
    }
}
