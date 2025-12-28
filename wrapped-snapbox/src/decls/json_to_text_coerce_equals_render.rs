macro_rules! deps {
    () => {
        DataFormat!();
        Data!();
    };
}

macro_rules! json_to_text_coerce_equals_render {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_to_text_coerce_equals_render () { let json = json ! ({ "name" : "John\\Doe\r\n" }) ; let d = Data :: json (json) ; let text = d . clone () . coerce_to (DataFormat :: Text) ; assert_eq ! (Data :: text (d . render () . unwrap ()) , text) ; }
    };
}

json_to_text_coerce_equals_render!();