macro_rules! deps {
    () => {
        Result!();
        Error!();
        Filter!();
    };
}

macro_rules! trim_start_matches {
    () => {
        deps!();
        # [doc = " Strip leading characters that match the given pattern."] pub fn trim_start_matches (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("trim_start_matches" , "value" , String , value) ; let pat = match args . get ("pat") { Some (pat) => { let p = try_get_value ! ("trim_start_matches" , "pat" , String , pat) ; p . replace ("\\n" , "\n") . replace ("\\t" , "\t") } None => return Err (Error :: msg ("Filter `trim_start_matches` expected an arg called `pat`")) , } ; Ok (to_value (s . trim_start_matches (& pat)) . unwrap ()) }
    };
}

trim_start_matches!()