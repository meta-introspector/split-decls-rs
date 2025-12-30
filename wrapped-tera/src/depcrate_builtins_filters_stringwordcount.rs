// Generated macro for wordcount (function)
macro_rules! Depcrate_builtins_filters_stringwordcount {
() => {
// Module: crate::builtins::filters::string
// Provides: {"wordcount"}
// Dependencies: {}
# [doc = " Gets the number of words in a string."] pub fn wordcount (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("wordcount" , "value" , String , value) ; Ok (to_value (s . split_whitespace () . count ()) . unwrap ()) }
};
}
