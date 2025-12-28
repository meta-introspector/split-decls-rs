macro_rules! deps {
    () => {
        List!();
        DiagnosticDeriveError!();
    };
}

macro_rules! report_type_error {
    () => {
        deps!();
        # [doc = " Reports a type error for field with `attr`."] pub (crate) fn report_type_error (attr : & Attribute , ty_name : & str ,) -> Result < ! , DiagnosticDeriveError > { let name = attr . path () . segments . last () . unwrap () . ident . to_string () ; let meta = & attr . meta ; throw_span_err ! (attr . span () . unwrap () , & format ! ("the `#[{}{}]` attribute can only be applied to fields of type {}" , name , match meta { Meta :: Path (_) => "" , Meta :: NameValue (_) => " = ..." , Meta :: List (_) => "(...)" , } , ty_name)) ; }
    };
}

report_type_error!()