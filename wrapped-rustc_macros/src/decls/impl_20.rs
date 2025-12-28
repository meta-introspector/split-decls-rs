macro_rules! deps {
    () => {
        DiagnosticDeriveError!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl From < SynError > for DiagnosticDeriveError { fn from (e : SynError) -> Self { DiagnosticDeriveError :: SynError (e) } }
    };
}

impl_20!()