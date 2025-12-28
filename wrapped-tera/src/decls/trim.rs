macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! trim {
    () => {
        deps!();
        # [doc = " Strip leading and trailing whitespace."] pub fn trim (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("trim" , "value" , String , value) ; Ok (to_value (s . trim ()) . unwrap ()) }
    };
}

trim!()