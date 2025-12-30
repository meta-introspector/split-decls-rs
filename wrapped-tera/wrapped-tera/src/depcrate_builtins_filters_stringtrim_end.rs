// Generated macro for trim_end (function)
macro_rules! Depcrate_builtins_filters_stringtrim_end {
() => {
// Module: crate::builtins::filters::string
// Provides: {"trim_end"}
// Dependencies: {}
# [doc = " Strip trailing whitespace."] pub fn trim_end (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("trim_end" , "value" , String , value) ; Ok (to_value (s . trim_end ()) . unwrap ()) }
};
}
