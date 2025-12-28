macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! upper {
    () => {
        deps!();
        # [doc = " Convert a value to uppercase."] pub fn upper (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("upper" , "value" , String , value) ; Ok (to_value (s . to_uppercase ()) . unwrap ()) }
    };
}

upper!();