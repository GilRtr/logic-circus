use std::ops::{BitAnd, BitOr, BitXor, Not};

use crate::{util::ContainerSizeGames, Bit, GateLike, RustImpls};

#[cfg(test)]
mod tests;

impl<Rust> GateLike for RustImpls<Rust>
where
    Rust: GateLike,
{
    fn compute(&mut self, mut input: Vec<Bit>) -> Vec<Bit> {
        debug_assert_eq!(input.len(), self.num_of_inputs());

        unsafe {
            match self {
                RustImpls::Dup(ref amount) => {
                    let value = input.get_copied(0);
                    input.change_len(*amount, value)
                }
                RustImpls::Mem(current) => {
                    std::mem::swap(current, input.get_unchecked_mut(0));
                    input
                }
                RustImpls::Not => {
                    input.get_unchecked_mut(0).action_assign(Bit::not);
                    input
                }
                RustImpls::Nand => compute_binary_operation(input, |x, y| !(x && y)),
                RustImpls::And => compute_binary_operation(input, Bit::bitand),
                RustImpls::Or => compute_binary_operation(input, Bit::bitor),
                RustImpls::Nor => compute_binary_operation(input, |x, y| !(x || y)),
                RustImpls::Xor => compute_binary_operation(input, Bit::bitxor),
                RustImpls::User(component) => component.compute(input),
            }
        }
    }

    fn num_of_inputs(&self) -> usize {
        match self {
            RustImpls::Dup(_) => 1,
            RustImpls::Mem(_) => 1,
            RustImpls::Not => 1,
            RustImpls::Nand => 2,
            RustImpls::And => 2,
            RustImpls::Or => 2,
            RustImpls::Nor => 2,
            RustImpls::Xor => 2,
            RustImpls::User(component) => component.num_of_inputs(),
        }
    }
}

unsafe fn compute_binary_operation<O>(input: Vec<Bit>, operation: O) -> Vec<Bit>
where
    O: FnOnce(Bit, Bit) -> Bit,
{
    let value = operation(input.get_copied(0), input.get_copied(1));
    input.shorten(1, value)
}

trait ActionAssign: Sized + Copy {
    fn action_assign<F>(&mut self, action: F) -> &mut Self
    where
        F: FnOnce(Self) -> Self,
    {
        *self = action(*self);
        self
    }
}
impl<T> ActionAssign for T where T: Sized + Copy {}
