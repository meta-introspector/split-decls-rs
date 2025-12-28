macro_rules! deps {
    () => {
        Error!();
        Filter!();
        Result!();
    };
}

macro_rules! round {
    () => {
        deps!();
        # [doc = " Returns a rounded number using the `method` arg and `precision` given."] # [doc = " `method` defaults to `common` which will round to the nearest number."] # [doc = " `ceil` and `floor` are also available as method."] # [doc = " `precision` defaults to `0`, meaning it will round to an integer"] pub fn round (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let num = try_get_value ! ("round" , "value" , f64 , value) ; let method = match args . get ("method") { Some (val) => try_get_value ! ("round" , "method" , String , val) , None => "common" . to_string () , } ; let precision = match args . get ("precision") { Some (val) => try_get_value ! ("round" , "precision" , i32 , val) , None => 0 , } ; let multiplier = if precision == 0 { 1.0 } else { 10.0_f64 . powi (precision) } ; match method . as_ref () { "common" => Ok (to_value ((multiplier * num) . round () / multiplier) . unwrap ()) , "ceil" => Ok (to_value ((multiplier * num) . ceil () / multiplier) . unwrap ()) , "floor" => Ok (to_value ((multiplier * num) . floor () / multiplier) . unwrap ()) , _ => Err (Error :: msg (format ! ("Filter `round` received an incorrect value for arg `method`: got `{:?}`, \
             only common, ceil and floor are allowed" , method))) , } }
    };
}

round!()