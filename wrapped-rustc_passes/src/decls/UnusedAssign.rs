macro_rules! deps {
    () => {
        UnusedAssignSuggestion!();
    };
}

macro_rules! UnusedAssign {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (passes_unused_assign)] pub (crate) struct UnusedAssign { pub name : String , # [subdiagnostic] pub suggestion : Option < UnusedAssignSuggestion > , # [help] pub help : bool , }
    };
}

UnusedAssign!()