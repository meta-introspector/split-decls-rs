macro_rules! UnsafeNotInheritedLintNote {
    () => {
        pub (crate) struct UnsafeNotInheritedLintNote { pub (crate) signature_span : Span , pub (crate) body_span : Span , }
    };
}

UnsafeNotInheritedLintNote!()