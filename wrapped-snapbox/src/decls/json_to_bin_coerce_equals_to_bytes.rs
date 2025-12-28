macro_rules! deps {
    () => {
        Data!();
        DataFormat!();
    };
}

macro_rules! json_to_bin_coerce_equals_to_bytes {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_to_bin_coerce_equals_to_bytes () { let json = json ! ({ "name" : "John\\Doe\r\n" }) ; let d = Data :: json (json) ; let binary = d . clone () . coerce_to (DataFormat :: Binary) ; assert_eq ! (Data :: binary (d . to_bytes () . unwrap ()) , binary) ; }
    };
}

json_to_bin_coerce_equals_to_bytes!()