// Generated macro for lower (function)
macro_rules! Depcrate_builtins_filters_stringlower {
() => {
// Module: crate::builtins::filters::string
// Provides: {"lower"}
// Dependencies: {}
# [doc = " Convert a value to lowercase."] pub fn lower (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("lower" , "value" , String , value) ; Ok (to_value (s . to_lowercase ()) . unwrap ()) }
};
}
