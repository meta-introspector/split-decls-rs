// Generated macro for striptags (function)
macro_rules! Depcrate_builtins_filters_stringstriptags {
() => {
// Module: crate::builtins::filters::string
// Provides: {"striptags"}
// Dependencies: {}
# [doc = " Removes html tags from string"] pub fn striptags (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("striptags" , "value" , String , value) ; Ok (to_value (STRIPTAGS_RE . replace_all (& s , "")) . unwrap ()) }
};
}
