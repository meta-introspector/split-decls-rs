macro_rules! UselessAssignment {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_useless_assignment)] pub (crate) struct UselessAssignment < 'a > { pub is_field_assign : bool , pub ty : Ty < 'a > , }
    };
}

UselessAssignment!()