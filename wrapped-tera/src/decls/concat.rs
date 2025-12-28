macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! concat {
    () => {
        deps!();
        # [doc = " Concat the array with another one if the `with` parameter is an array or"] # [doc = " just append it otherwise"] pub fn concat (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let mut arr = try_get_value ! ("concat" , "value" , Vec < Value >, value) ; let value = match args . get ("with") { Some (val) => val , None => return Err (Error :: msg ("The `concat` filter has to have a `with` argument")) , } ; if value . is_array () { match value { Value :: Array (vals) => { for val in vals { arr . push (val . clone ()) ; } } _ => unreachable ! ("Got something other than an array??") , } } else { arr . push (value . clone ()) ; } Ok (to_value (arr) . unwrap ()) }
    };
}

concat!();