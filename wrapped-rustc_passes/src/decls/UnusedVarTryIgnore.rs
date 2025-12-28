macro_rules! deps {
    () => {
        UnusedVarTryIgnoreSugg!();
    };
}

macro_rules! UnusedVarTryIgnore {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (passes_unused_variable_try_ignore)] pub (crate) struct UnusedVarTryIgnore { pub name : String , # [subdiagnostic] pub sugg : UnusedVarTryIgnoreSugg , }
    };
}

UnusedVarTryIgnore!();