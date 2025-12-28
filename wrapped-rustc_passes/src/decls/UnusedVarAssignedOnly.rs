macro_rules! deps {
    () => {
        PatternTypo!();
    };
}

macro_rules! UnusedVarAssignedOnly {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (passes_unused_var_assigned_only)] # [note] pub (crate) struct UnusedVarAssignedOnly { pub name : String , # [subdiagnostic] pub typo : Option < PatternTypo > , }
    };
}

UnusedVarAssignedOnly!();