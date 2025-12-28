macro_rules! deps {
    () => {
        FatalError!();
        ErrorGuaranteed!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        impl ErrorGuaranteed { # [doc = " Don't use this outside of `DiagCtxtInner::emit_diagnostic`!"] # [deprecated = "should only be used in `DiagCtxtInner::emit_diagnostic`"] pub fn unchecked_error_guaranteed () -> Self { ErrorGuaranteed (()) } pub fn raise_fatal (self) -> ! { FatalError . raise () } }
    };
}

impl_343!()