// Generated macro for int (function)
macro_rules! Depcrate_builtins_filters_stringint {
() => {
// Module: crate::builtins::filters::string
// Provides: {"int"}
// Dependencies: {}
# [doc = " Convert the value to a signed integer number"] pub fn int (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let default = match args . get ("default") { Some (d) => try_get_value ! ("int" , "default" , i64 , d) , None => 0 , } ; let base = match args . get ("base") { Some (b) => try_get_value ! ("int" , "base" , u32 , b) , None => 10 , } ; let v = match value { Value :: String (s) => { let s = s . trim () ; let s = match base { 2 => s . trim_start_matches ("0b") , 8 => s . trim_start_matches ("0o") , 16 => s . trim_start_matches ("0x") , _ => s , } ; match i64 :: from_str_radix (s , base) { Ok (v) => v , Err (_) => { if s . contains ('.') { match s . parse :: < f64 > () { Ok (f) => f as i64 , Err (_) => default , } } else { default } } } } Value :: Number (n) => match n . as_f64 () { Some (f) => f as i64 , None => match n . as_i64 () { Some (i) => i , None => default , } , } , _ => return Err (Error :: msg ("Filter `int` received an unexpected type")) , } ; Ok (to_value (v) . unwrap ()) }
};
}
