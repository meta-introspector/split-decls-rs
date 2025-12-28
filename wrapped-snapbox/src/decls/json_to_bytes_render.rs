macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! json_to_bytes_render {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_to_bytes_render () { let d = Data :: json (json ! ({ "name" : "John\\Doe\r\n" })) ; let bytes = d . to_bytes () . unwrap () ; let bytes = String :: from_utf8 (bytes) . unwrap () ; let rendered = d . render () . unwrap () ; assert_eq ! (bytes , rendered) ; }
    };
}

json_to_bytes_render!()