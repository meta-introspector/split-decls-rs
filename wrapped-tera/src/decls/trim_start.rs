macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! trim_start {
    () => {
        deps!();
        # [doc = " Strip leading whitespace."] pub fn trim_start (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("trim_start" , "value" , String , value) ; Ok (to_value (s . trim_start ()) . unwrap ()) }
    };
}

trim_start!()