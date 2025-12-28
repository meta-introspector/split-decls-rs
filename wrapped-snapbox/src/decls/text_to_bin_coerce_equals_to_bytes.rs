macro_rules! deps {
    () => {
        DataFormat!();
        Data!();
    };
}

macro_rules! text_to_bin_coerce_equals_to_bytes {
    () => {
        deps!();
        # [test] fn text_to_bin_coerce_equals_to_bytes () { let text = String :: from ("test") ; let d = Data :: text (text) ; let binary = d . clone () . coerce_to (DataFormat :: Binary) ; assert_eq ! (Data :: binary (d . to_bytes () . unwrap ()) , binary) ; }
    };
}

text_to_bin_coerce_equals_to_bytes!()