macro_rules! deps {
    () => {
        FieldInfo!();
        DiagnosticDeriveError!();
    };
}

macro_rules! report_error_if_not_applied_to_ty {
    () => {
        deps!();
        # [doc = " Reports an error if the field's type does not match `path`."] fn report_error_if_not_applied_to_ty (attr : & Attribute , info : & FieldInfo < '_ > , path : & [& str] , ty_name : & str ,) -> Result < () , DiagnosticDeriveError > { if ! type_matches_path (info . ty . inner_type () , path) { report_type_error (attr , ty_name) ? ; } Ok (()) }
    };
}

report_error_if_not_applied_to_ty!();