// Generated macro for upper (function)
macro_rules! Depcrate_builtins_filters_stringupper {
() => {
// Module: crate::builtins::filters::string
// Provides: {"upper"}
// Dependencies: {}
# [doc = " Convert a value to uppercase."] pub fn upper (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("upper" , "value" , String , value) ; Ok (to_value (s . to_uppercase ()) . unwrap ()) }
};
}
