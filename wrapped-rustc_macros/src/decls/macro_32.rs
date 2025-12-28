macro_rules! macro_32 {
    () => {
        decl_derive ! ([Diagnostic , attributes (diag , help , help_once , note , note_once , warning , skip_arg , primary_span , label , subdiagnostic , suggestion , suggestion_short , suggestion_hidden , suggestion_verbose)] => diagnostics :: diagnostic_derive) ;
    };
}

macro_32!()