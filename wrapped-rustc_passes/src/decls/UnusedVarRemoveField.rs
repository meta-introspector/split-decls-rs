macro_rules! deps {
    () => {
        UnusedVarRemoveFieldSugg!();
    };
}

macro_rules! UnusedVarRemoveField {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (passes_unused_var_remove_field)] pub (crate) struct UnusedVarRemoveField { pub name : String , # [subdiagnostic] pub sugg : UnusedVarRemoveFieldSugg , }
    };
}

UnusedVarRemoveField!()