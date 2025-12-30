// Generated macro for spaceless (function)
macro_rules! Depcrate_builtins_filters_stringspaceless {
() => {
// Module: crate::builtins::filters::string
// Provides: {"spaceless"}
// Dependencies: {}
# [doc = " Removes spaces between html tags from string"] pub fn spaceless (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("spaceless" , "value" , String , value) ; Ok (to_value (SPACELESS_RE . replace_all (& s , "><")) . unwrap ()) }
};
}
