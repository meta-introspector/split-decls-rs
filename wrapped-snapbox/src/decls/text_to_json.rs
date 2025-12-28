macro_rules! deps {
    () => {
        DataFormat!();
        Data!();
    };
}

macro_rules! text_to_json {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn text_to_json () { let value = json ! ({ "name" : "John\\Doe\r\n" }) ; let text = serde_json :: to_string_pretty (& value) . unwrap () ; let d = Data :: text (text) ; let json = d . coerce_to (DataFormat :: Json) ; assert_eq ! (DataFormat :: Json , json . format ()) ; }
    };
}

text_to_json!()