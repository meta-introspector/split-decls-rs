macro_rules! macro_169 {
    () => {
        decl_derive ! ([LintDiagnostic , attributes (diag , help , help_once , note , note_once , warning , skip_arg , primary_span , label , subdiagnostic , suggestion , suggestion_short , suggestion_hidden , suggestion_verbose)] => diagnostics :: lint_diagnostic_derive) ;
    };
}

macro_169!();