#[macro_export]
macro_rules! sequal {
    ($index:expr => $entry:expr) => {{
        Sequal::gate($index, $entry)
    }};
    ($entry:expr) => {{
        Sequal::end($entry)
    }};
}

#[macro_export]
macro_rules! sequals {
    ($index:expr => $entry:expr $(, $($tail:tt)+)? $(,)?) => {{
        vec![Sequal::gate($index, $entry) $(, sequals!(@rest $($tail)+))?]
    }};
    ($entry:expr $(, $($tail:tt)+)? $(,)?) => {{
        vec![Sequal::end($entry) $(, sequals!(@rest $($tail)+))?]
    }};
    (@rest $index:expr => $entry:expr $(, $($tail:tt)+)? $(,)?) => {
        Sequal::gate($index, $entry) $(, sequals!(@rest $($tail)+))?
    };
    (@rest $entry:expr $(, $($tail:tt)+)? $(,)?) => {
        Sequal::end($entry) $(, sequals!(@rest $($tail)+))?
    };
}

#[macro_export]
macro_rules! comp {
    (($index:expr => $entry:expr $(, $($sequal_tail:tt)+)?) ; $($tail:tt)*) => {{
        let mut __gates = Vec::new();
        let mut __sequals = Vec::new();
        comp!(@rest __sequals __gates $($tail)*);
        $crate::Component::new(__gates, sequals![$index => $entry $(, $($sequal_tail)+)?], __sequals)
    }};
    ($index:expr => $entry:expr ; $($tail:tt)*) => {{
        let mut __gates = Vec::new();
        let mut __sequals = Vec::new();
        comp!(@rest __sequals __gates $($tail)*);
        $crate::Component::new(__gates, sequals![$index => $entry], __sequals)
    }};
    (($entry:expr $(, $($sequal_tail:tt)+)?) ; $($tail:tt)*) => {{
        let mut __gates = Vec::new();
        let mut __sequals = Vec::new();
        comp!(@rest $__sequals $__gates $($tail)*);
        $crate::Component::new(__gates, sequals![$entry $(, $($sequal_tail)+)?], __sequals)
    }};
    ($entry:expr ; $($tail:tt)*) => {{
        let mut __gates = Vec::new();
        let mut __sequals = Vec::new();
        comp!(@rest $__sequals $__gates $($tail)*);
        $crate::Component::new(__gates, sequals![$entry], __sequals)
    }};
    (@rest $__sequals:ident $__gates:ident dup: ($index:expr => $entry:expr $(, $($sequal_tail:tt)+)?) ; $($tail:tt)*) => {
        $__sequals.push(sequals![$index => $entry $(, $($sequal_tail)+)?]);
        $__gates.push($crate::Gate::duplicator(unsafe { $__sequals.get_unchecked($__sequals.len() - 1) }.len()));
        comp!(@rest $__sequals $__gates $($tail)*);
    };
    (@rest $__sequals:ident $__gates:ident dup: $index:expr => $entry:expr ; $($tail:tt)*) => {
        $__sequals.push(sequals![$index => $entry]);
        $__gates.push($crate::Gate::duplicator(unsafe { $__sequals.get_unchecked($__sequals.len() - 1) }.len()));
        comp!(@rest $__sequals $__gates $($tail)*);
    };
    (@rest $__sequals:ident $__gates:ident dup: ($entry:expr $(, $($sequal_tail:tt)+)?) ; $($tail:tt)*) => {
        $__sequals.push(sequals![$entry $(, $($sequal_tail)+)?]);
        $__gates.push($crate::Gate::duplicator(unsafe { $__sequals.get_unchecked($__sequals.len() - 1) }.len()));
        comp!(@rest $__sequals $__gates $($tail)*);
    };
    (@rest $__sequals:ident $__gates:ident dup: $entry:expr ; $($tail:tt)*) => {
        $__sequals.push(sequals![$entry]);
        $__gates.push($crate::Gate::duplicator(unsafe { $__sequals.get_unchecked($__sequals.len() - 1) }.len()));
        comp!(@rest $__sequals $__gates $($tail)*);
    };
    (@rest $__sequals:ident $__gates:ident nand: $index:expr => $entry:expr ; $($tail:tt)*) => {
        $__sequals.push(sequals![$index => $entry]);
        $__gates.push($crate::Gate::nand_gate());
        comp!(@rest $__sequals $__gates $($tail)*);
    };
    (@rest $__sequals:ident $__gates:ident nand: $entry:expr ; $($tail:tt)*) => {
        $__sequals.push(sequals![$entry]);
        $__gates.push($crate::Gate::nand_gate());
        comp!(@rest $__sequals $__gates $($tail)*);
    };
    (@rest $__sequals:ident $__gates:ident $component:ident: ($index:expr => $entry:expr $(, $($sequal_tail:tt)+)?) ; $($tail:tt)*) => {
        $__sequals.push(sequals![$index => $entry $(, $($sequal_tail)+)?]);
        $__gates.push($crate::Gate::custom($component.clone()));
        comp!(@rest $__sequals $__gates $($tail)*);
    };
    (@rest $__sequals:ident $__gates:ident $component:ident: $index:expr => $entry:expr ; $($tail:tt)*) => {
        $__sequals.push(sequals![$index => $entry]);
        $__gates.push($crate::Gate::custom($component.clone()));
        comp!(@rest $__sequals $__gates $($tail)*);
    };
    (@rest $__sequals:ident $__gates:ident $component:ident: ($entry:expr $(, $($sequal_tail:tt)+)?) ; $($tail:tt)*) => {
        $__sequals.push(sequals![$entry $(, $($sequal_tail)+)?]);
        $__gates.push($crate::Gate::custom($component,clone()));
        comp!(@rest $__sequals $__gates $($tail)*);
    };
    (@rest $__sequals:ident $__gates:ident $component:ident: $entry:expr ; $($tail:tt)*) => {
        $__sequals.push(sequals![$entry]);
        $__gates.push($crate::Gate::custom($component.clone()));
        comp!(@rest $__sequals $__gates $($tail)*);
    };
    (@rest $__sequals:ident $__gates:ident) => {};
}
