macro_rules! deps {
    () => {
        Error!();
        Filter!();
        Result!();
    };
}

macro_rules! replace {
    () => {
        deps!();
        # [doc = " Replaces given `from` substring with `to` string."] pub fn replace (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("replace" , "value" , String , value) ; let from = match args . get ("from") { Some (val) => try_get_value ! ("replace" , "from" , String , val) , None => return Err (Error :: msg ("Filter `replace` expected an arg called `from`")) , } ; let to = match args . get ("to") { Some (val) => try_get_value ! ("replace" , "to" , String , val) , None => return Err (Error :: msg ("Filter `replace` expected an arg called `to`")) , } ; Ok (to_value (s . replace (& from , & to)) . unwrap ()) }
    };
}

replace!()