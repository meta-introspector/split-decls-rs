macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! trim_end {
    () => {
        deps!();
        # [doc = " Strip trailing whitespace."] pub fn trim_end (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("trim_end" , "value" , String , value) ; Ok (to_value (s . trim_end ()) . unwrap ()) }
    };
}

trim_end!()