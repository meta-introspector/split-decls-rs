macro_rules! deps {
    () => {
        AssertLintKind!();
        Lint!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl AssertLintKind { pub (crate) fn lint (& self) -> & 'static Lint { match self { AssertLintKind :: ArithmeticOverflow => lint :: builtin :: ARITHMETIC_OVERFLOW , AssertLintKind :: UnconditionalPanic => lint :: builtin :: UNCONDITIONAL_PANIC , } } }
    };
}

impl_65!()