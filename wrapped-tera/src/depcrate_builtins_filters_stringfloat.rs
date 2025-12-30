// Generated macro for float (function)
macro_rules! Depcrate_builtins_filters_stringfloat {
() => {
// Module: crate::builtins::filters::string
// Provides: {"float"}
// Dependencies: {}
# [doc = " Convert the value to a floating point number"] pub fn float (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let default = match args . get ("default") { Some (d) => try_get_value ! ("float" , "default" , f64 , d) , None => 0.0 , } ; let v = match value { Value :: String (s) => { let s = s . trim () ; s . parse :: < f64 > () . unwrap_or (default) } Value :: Number (n) => match n . as_f64 () { Some (f) => f , None => match n . as_i64 () { Some (i) => i as f64 , None => default , } , } , _ => return Err (Error :: msg ("Filter `float` received an unexpected type")) , } ; Ok (to_value (v) . unwrap ()) }
};
}
