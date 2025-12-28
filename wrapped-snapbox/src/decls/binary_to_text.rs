macro_rules! deps {
    () => {
        DataFormat!();
        Data!();
    };
}

macro_rules! binary_to_text {
    () => {
        deps!();
        # [test] fn binary_to_text () { let binary = String :: from ("test") . into_bytes () ; let d = Data :: binary (binary) ; let text = d . coerce_to (DataFormat :: Text) ; assert_eq ! (DataFormat :: Text , text . format ()) ; }
    };
}

binary_to_text!()