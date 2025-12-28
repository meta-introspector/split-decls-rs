macro_rules! AssertLintKind {
    () => {
        pub (crate) enum AssertLintKind { ArithmeticOverflow , UnconditionalPanic , }
    };
}

AssertLintKind!()