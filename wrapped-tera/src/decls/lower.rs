macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! lower {
    () => {
        deps!();
        # [doc = " Convert a value to lowercase."] pub fn lower (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("lower" , "value" , String , value) ; Ok (to_value (s . to_lowercase ()) . unwrap ()) }
    };
}

lower!();