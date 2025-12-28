macro_rules! deps {
    () => {
        AssertLintKind!();
    };
}

macro_rules! AssertLint {
    () => {
        deps!();
        pub (crate) struct AssertLint < P > { pub span : Span , pub assert_kind : AssertKind < P > , pub lint_kind : AssertLintKind , }
    };
}

AssertLint!()