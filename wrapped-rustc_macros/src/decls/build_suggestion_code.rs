macro_rules! deps {
    () => {
        AllowMultipleAlternatives!();
        HasFieldMap!();
    };
}

macro_rules! build_suggestion_code {
    () => {
        deps!();
        # [doc = " Constructs the `format!()` invocation(s) necessary for a `#[suggestion*(code = \"foo\")]` or"] # [doc = " `#[suggestion*(code(\"foo\", \"bar\"))]` attribute field"] pub (super) fn build_suggestion_code (code_field : & Ident , nested : ParseNestedMeta < '_ > , fields : & impl HasFieldMap , allow_multiple : AllowMultipleAlternatives ,) -> TokenStream { let values = match parse_suggestion_values (nested , allow_multiple) { Ok (x) => x , Err (e) => return e . into_compile_error () , } ; if let AllowMultipleAlternatives :: Yes = allow_multiple { let formatted_strings : Vec < _ > = values . into_iter () . map (| value | fields . build_format (& value . value () , value . span ())) . collect () ; quote ! { let # code_field = [# (# formatted_strings) ,*] . into_iter () ; } } else if let [value] = values . as_slice () { let formatted_str = fields . build_format (& value . value () , value . span ()) ; quote ! { let # code_field = # formatted_str ; } } else { quote ! { let # code_field = String :: new () ; } } }
    };
}

build_suggestion_code!();