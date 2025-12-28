macro_rules! deps {
    () => {
        Data!();
        DataFormat!();
    };
}

macro_rules! binary_to_json_not_json {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn binary_to_json_not_json () { let binary = String :: from ("test") . into_bytes () ; let d = Data :: binary (binary) ; let d = d . coerce_to (DataFormat :: Json) ; assert_ne ! (DataFormat :: Json , d . format ()) ; assert_eq ! (DataFormat :: Binary , d . format ()) ; }
    };
}

binary_to_json_not_json!();