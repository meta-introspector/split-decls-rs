macro_rules! deps {
    () => {
        SyntaxError!();
    };
}

macro_rules! validate_macro_rules {
    () => {
        deps!();
        fn validate_macro_rules (mac : ast :: MacroRules , errors : & mut Vec < SyntaxError >) { if let Some (vis) = mac . visibility () { errors . push (SyntaxError :: new ("visibilities are not allowed on `macro_rules!` items" , vis . syntax () . text_range () ,)) ; } }
    };
}

validate_macro_rules!()