// Generated macro for addslashes (function)
macro_rules! Depcrate_builtins_filters_stringaddslashes {
() => {
// Module: crate::builtins::filters::string
// Provides: {"addslashes"}
// Dependencies: {}
# [doc = " Escapes quote characters"] pub fn addslashes (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("addslashes" , "value" , String , value) ; Ok (to_value (s . replace ('\\' , "\\\\") . replace ('\"' , "\\\"") . replace ('\'' , "\\\'")) . unwrap ()) }
};
}
