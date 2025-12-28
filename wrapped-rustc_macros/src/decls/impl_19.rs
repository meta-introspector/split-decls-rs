macro_rules! deps {
    () => {
        DiagnosticDeriveError!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl DiagnosticDeriveError { pub (crate) fn to_compile_error (self) -> TokenStream { match self { DiagnosticDeriveError :: SynError (e) => e . to_compile_error () , DiagnosticDeriveError :: ErrorHandled => { quote ! { { unreachable ! () ; } } } } } }
    };
}

impl_19!()