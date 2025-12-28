macro_rules! deps {
    () => {
        Data!();
        DataFormat!();
    };
}

macro_rules! json_to_binary {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_to_binary () { let value = json ! ({ "name" : "John\\Doe\r\n" }) ; let d = Data :: json (value) ; let binary = d . coerce_to (DataFormat :: Binary) ; assert_eq ! (DataFormat :: Binary , binary . format ()) ; }
    };
}

json_to_binary!()