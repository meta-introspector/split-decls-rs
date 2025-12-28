macro_rules! deps {
    () => {
        Data!();
        DataFormat!();
    };
}

macro_rules! json_to_text {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_to_text () { let value = json ! ({ "name" : "John\\Doe\r\n" }) ; let d = Data :: json (value) ; let text = d . coerce_to (DataFormat :: Text) ; assert_eq ! (DataFormat :: Text , text . format ()) ; }
    };
}

json_to_text!();