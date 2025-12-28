macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! addslashes {
    () => {
        deps!();
        # [doc = " Escapes quote characters"] pub fn addslashes (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("addslashes" , "value" , String , value) ; Ok (to_value (s . replace ('\\' , "\\\\") . replace ('\"' , "\\\"") . replace ('\'' , "\\\'")) . unwrap ()) }
    };
}

addslashes!();