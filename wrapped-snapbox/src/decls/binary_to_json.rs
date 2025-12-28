macro_rules! deps {
    () => {
        DataFormat!();
        Data!();
    };
}

macro_rules! binary_to_json {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn binary_to_json () { let value = json ! ({ "name" : "John\\Doe\r\n" }) ; let binary = serde_json :: to_vec_pretty (& value) . unwrap () ; let d = Data :: binary (binary) ; let json = d . coerce_to (DataFormat :: Json) ; assert_eq ! (DataFormat :: Json , json . format ()) ; }
    };
}

binary_to_json!();