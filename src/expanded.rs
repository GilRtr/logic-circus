use sequals::SequalsExtension;
mod gate {
    use crate::{zeroed_vec, Bit, Gate, GateKind};
    use std::mem;
    impl Gate {
        pub(super) fn fill_input(&mut self, with: Bit, at: usize) -> Option<Vec<Bit>> {
            if true {
                if !(self.inputs.len() > at) {
                    ::core::panicking::panic("assertion failed: self.inputs.len() > at")
                };
            };
            *unsafe { self.inputs.get_unchecked_mut(at) } = with;
            self.inputs_filled += 1;
            (self.inputs_filled == self.inputs.len()).then(|| self.compute())
        }
        fn compute(&mut self) -> Vec<Bit> {
            let mut inputs = mem::take(&mut self.inputs);
            match &mut self.kind {
                GateKind::Nand => {
                    if true {
                        {
                            match (&inputs.len(), &2) {
                                (left_val, right_val) => {
                                    if !(*left_val == *right_val) {
                                        let kind = ::core::panicking::AssertKind::Eq;
                                        ::core::panicking::assert_failed(
                                            kind,
                                            &*left_val,
                                            &*right_val,
                                            ::core::option::Option::None,
                                        );
                                    }
                                }
                            }
                        };
                    };
                    inputs.clear();
                    inputs.push(!(unsafe { *inputs.get_unchecked(0) && *inputs.get_unchecked(1) }));
                    inputs
                }
                GateKind::Duplicate(ref amount) => {
                    if true {
                        {
                            match (&inputs.len(), &1) {
                                (left_val, right_val) => {
                                    if !(*left_val == *right_val) {
                                        let kind = ::core::panicking::AssertKind::Eq;
                                        ::core::panicking::assert_failed(
                                            kind,
                                            &*left_val,
                                            &*right_val,
                                            ::core::option::Option::None,
                                        );
                                    }
                                }
                            }
                        };
                    };
                    inputs.resize(*amount, *unsafe { inputs.get_unchecked(0) });
                    inputs
                }
                GateKind::Custom(component) => component.run(inputs),
            }
        }
        pub(super) fn reset(&mut self) {
            self.inputs_filled = 0;
            self.inputs = zeroed_vec(self.kind.get_len());
            if let GateKind::Custom(inner) = &mut self.kind {
                inner.reset();
            }
        }
    }
}
mod sequals {
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
            if true {
                {
                    match (&sequals.len(), &output.len()) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::Some(::core::fmt::Arguments::new_v1(
                                        &["index is "],
                                        &match (&for_gate,) {
                                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            )],
                                        },
                                    )),
                                );
                            }
                        }
                    }
                };
            };
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
}
mod components {
    use super::*;
    fn not() {
        let mut not = {
            let mut __gates = Vec::new();
            let mut __sequals = Vec::new();
            {
                __sequals.push({ <[_]>::into_vec(box [Sequal::gate(1, 0), Sequal::gate(1, 1)]) });
                __gates.push(crate::Gate::duplicator(
                    unsafe { __sequals.get_unchecked(__sequals.len() - 1) }.len(),
                ));
                {
                    __sequals.push({ <[_]>::into_vec(box [Sequal::end(0)]) });
                    __gates.push(crate::Gate::nand_gate());
                };
            };
            crate::Component::new(
                __gates,
                { <[_]>::into_vec(box [Sequal::gate(0, 0)]) },
                __sequals,
            )
        };
        {
            match (
                &not.run(<[_]>::into_vec(box [true])),
                &<[_]>::into_vec(box [false]),
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        not.reset();
        {
            match (
                &not.run(<[_]>::into_vec(box [false])),
                &<[_]>::into_vec(box [true]),
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
    }
}
type Bit = bool;
pub struct Component {
    gates: Vec<Gate>,
    /// for every output in each gate (`sequals[0][1]` is for the second output of the first gate)
    /// sequals.last
    sequals: Vec<Vec<Sequal>>,
    /// number of outputs
    outputs: usize,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Component {
    #[inline]
    fn clone(&self) -> Component {
        match *self {
            Component {
                gates: ref __self_0_0,
                sequals: ref __self_0_1,
                outputs: ref __self_0_2,
            } => Component {
                gates: ::core::clone::Clone::clone(&(*__self_0_0)),
                sequals: ::core::clone::Clone::clone(&(*__self_0_1)),
                outputs: ::core::clone::Clone::clone(&(*__self_0_2)),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Component {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Component {
                gates: ref __self_0_0,
                sequals: ref __self_0_1,
                outputs: ref __self_0_2,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Component");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "gates", &&(*__self_0_0));
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "sequals",
                    &&(*__self_0_1),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "outputs",
                    &&(*__self_0_2),
                );
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
impl Component {
    pub fn new(
        gates: Vec<Gate>,
        input_sequals: Vec<Sequal>,
        mut sequals: Vec<Vec<Sequal>>,
    ) -> Self {
        if true {
            {
                match (&gates.len(), &sequals.len()) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                }
            };
        };
        sequals.push(input_sequals);
        Self {
            outputs: sequals
                .iter()
                .flatten()
                .filter(|x| match x {
                    Sequal::End { .. } => true,
                    _ => false,
                })
                .count(),
            gates,
            sequals,
        }
    }
}
pub enum Sequal {
    Gate { index: usize, entry: usize },
    End { output: usize },
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Sequal {
    #[inline]
    fn clone(&self) -> Sequal {
        {
            let _: ::core::clone::AssertParamIsClone<usize>;
            let _: ::core::clone::AssertParamIsClone<usize>;
            let _: ::core::clone::AssertParamIsClone<usize>;
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for Sequal {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Sequal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Sequal::Gate {
                index: ref __self_0,
                entry: ref __self_1,
            },) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Gate");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "index", &&(*__self_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "entry", &&(*__self_1));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
            (&Sequal::End {
                output: ref __self_0,
            },) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "End");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "output", &&(*__self_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
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
pub struct Gate {
    kind: GateKind,
    inputs: Vec<Bit>,
    inputs_filled: usize,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Gate {
    #[inline]
    fn clone(&self) -> Gate {
        match *self {
            Gate {
                kind: ref __self_0_0,
                inputs: ref __self_0_1,
                inputs_filled: ref __self_0_2,
            } => Gate {
                kind: ::core::clone::Clone::clone(&(*__self_0_0)),
                inputs: ::core::clone::Clone::clone(&(*__self_0_1)),
                inputs_filled: ::core::clone::Clone::clone(&(*__self_0_2)),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Gate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Gate {
                kind: ref __self_0_0,
                inputs: ref __self_0_1,
                inputs_filled: ref __self_0_2,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Gate");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "kind", &&(*__self_0_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "inputs", &&(*__self_0_1));
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "inputs_filled",
                    &&(*__self_0_2),
                );
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
impl Gate {
    pub fn new(kind: GateKind) -> Self {
        Self {
            inputs: zeroed_vec(kind.get_len()),
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
fn zeroed_vec(len: usize) -> Vec<Bit> {
    let mut v = Vec::new();
    v.resize_with(len, Bit::default);
    v
}
pub enum GateKind {
    Nand,
    Duplicate(usize),
    Custom(Component),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for GateKind {
    #[inline]
    fn clone(&self) -> GateKind {
        match (&*self,) {
            (&GateKind::Nand,) => GateKind::Nand,
            (&GateKind::Duplicate(ref __self_0),) => {
                GateKind::Duplicate(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&GateKind::Custom(ref __self_0),) => {
                GateKind::Custom(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for GateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&GateKind::Nand,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Nand");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&GateKind::Duplicate(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Duplicate");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&GateKind::Custom(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Custom");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
impl GateKind {
    fn get_len(&self) -> usize {
        match self {
            Self::Nand => 2,
            Self::Duplicate(_) => 1,
            Self::Custom(component) => {
                unsafe { component.sequals.get_unchecked(component.sequals.len() - 1) }.len()
            }
        }
    }
}
impl Component {
    pub fn run(&mut self, input: Vec<Bit>) -> Vec<Bit> {
        let mut outputs = zeroed_vec(self.outputs);
        let mut outputs_filled = 0;
        let gates = &mut self.gates;
        self.sequals.run(
            self.sequals.len() - 1,
            input,
            gates,
            &mut outputs,
            &mut outputs_filled,
        );
        if true {
            {
                match (&outputs_filled, &outputs.len()) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                }
            };
        };
        outputs
    }
    pub fn reset(&mut self) {
        for gate in &mut self.gates {
            gate.reset()
        }
    }
}
