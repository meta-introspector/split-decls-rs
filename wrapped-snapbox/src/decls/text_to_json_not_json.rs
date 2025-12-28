macro_rules! deps {
    () => {
        Data!();
        DataFormat!();
    };
}

macro_rules! text_to_json_not_json {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn text_to_json_not_json () { let text = String :: from ("test") ; let d = Data :: text (text) ; let json = d . coerce_to (DataFormat :: Json) ; assert_eq ! (DataFormat :: Text , json . format ()) ; }
    };
}

text_to_json_not_json!()