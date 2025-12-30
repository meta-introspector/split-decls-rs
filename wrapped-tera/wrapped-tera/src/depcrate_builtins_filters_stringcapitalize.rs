// Generated macro for capitalize (function)
macro_rules! Depcrate_builtins_filters_stringcapitalize {
() => {
// Module: crate::builtins::filters::string
// Provides: {"capitalize"}
// Dependencies: {}
# [doc = " First letter of the string is uppercase rest is lowercase"] pub fn capitalize (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("capitalize" , "value" , String , value) ; let mut chars = s . chars () ; match chars . next () { None => Ok (to_value ("") . unwrap ()) , Some (f) => { let res = f . to_uppercase () . collect :: < String > () + & chars . as_str () . to_lowercase () ; Ok (to_value (res) . unwrap ()) } } }
};
}
