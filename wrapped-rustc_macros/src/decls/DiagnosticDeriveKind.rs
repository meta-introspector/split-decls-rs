macro_rules! DiagnosticDeriveKind {
    () => {
        # [doc = " What kind of diagnostic is being derived - a fatal/error/warning or a lint?"] # [derive (Clone , Copy , PartialEq , Eq)] pub (crate) enum DiagnosticDeriveKind { Diagnostic , LintDiagnostic , }
    };
}

DiagnosticDeriveKind!();