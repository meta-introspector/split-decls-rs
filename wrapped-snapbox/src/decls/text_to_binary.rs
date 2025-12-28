macro_rules! deps {
    () => {
        DataFormat!();
        Data!();
    };
}

macro_rules! text_to_binary {
    () => {
        deps!();
        # [test] fn text_to_binary () { let text = String :: from ("test") ; let d = Data :: text (text) ; let binary = d . coerce_to (DataFormat :: Binary) ; assert_eq ! (DataFormat :: Binary , binary . format ()) ; }
    };
}

text_to_binary!()