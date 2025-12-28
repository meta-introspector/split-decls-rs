macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! json_encode {
    () => {
        deps!();
        pub fn json_encode (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let pretty = args . get ("pretty") . and_then (Value :: as_bool) . unwrap_or (false) ; if pretty { to_string_pretty (& value) . map (Value :: String) . map_err (Error :: json) } else { to_string (& value) . map (Value :: String) . map_err (Error :: json) } }
    };
}

json_encode!()