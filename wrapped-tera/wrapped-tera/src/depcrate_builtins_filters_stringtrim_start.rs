// Generated macro for trim_start (function)
macro_rules! Depcrate_builtins_filters_stringtrim_start {
() => {
// Module: crate::builtins::filters::string
// Provides: {"trim_start"}
// Dependencies: {}
# [doc = " Strip leading whitespace."] pub fn trim_start (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("trim_start" , "value" , String , value) ; Ok (to_value (s . trim_start ()) . unwrap ()) }
};
}
