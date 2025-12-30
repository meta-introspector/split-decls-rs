// Generated macro for nth (function)
macro_rules! Depcrate_builtins_filters_arraynth {
() => {
// Module: crate::builtins::filters::array
// Provides: {"nth"}
// Dependencies: {}
# [doc = " Returns the nth value of an array"] # [doc = " If the array is empty, returns empty string"] pub fn nth (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let arr = try_get_value ! ("nth" , "value" , Vec < Value >, value) ; if arr . is_empty () { return Ok (to_value ("") . unwrap ()) ; } let index = match args . get ("n") { Some (val) => try_get_value ! ("nth" , "n" , usize , val) , None => return Err (Error :: msg ("The `nth` filter has to have an `n` argument")) , } ; Ok (arr . get (index) . unwrap_or (& to_value ("") . unwrap ()) . to_owned ()) }
};
}
