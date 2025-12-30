// Generated macro for trim (function)
macro_rules! Depcrate_builtins_filters_stringtrim {
() => {
// Module: crate::builtins::filters::string
// Provides: {"trim"}
// Dependencies: {}
# [doc = " Strip leading and trailing whitespace."] pub fn trim (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("trim" , "value" , String , value) ; Ok (to_value (s . trim ()) . unwrap ()) }
};
}
