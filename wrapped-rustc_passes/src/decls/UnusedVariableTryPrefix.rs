macro_rules! deps {
    () => {
        UnusedVariableStringInterp!();
        UnusedVariableSugg!();
        PatternTypo!();
    };
}

macro_rules! UnusedVariableTryPrefix {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (passes_unused_variable_try_prefix)] pub (crate) struct UnusedVariableTryPrefix { # [label] pub label : Option < Span > , # [subdiagnostic] pub string_interp : Vec < UnusedVariableStringInterp > , # [subdiagnostic] pub sugg : UnusedVariableSugg , pub name : String , # [subdiagnostic] pub typo : Option < PatternTypo > , }
    };
}

UnusedVariableTryPrefix!();