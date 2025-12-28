macro_rules! DiagnosticDeriveError {
    () => {
        # [derive (Debug)] pub (crate) enum DiagnosticDeriveError { SynError (SynError) , ErrorHandled , }
    };
}

DiagnosticDeriveError!()