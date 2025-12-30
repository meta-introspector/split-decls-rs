// Generated macro for last (function)
macro_rules! Depcrate_builtins_filters_arraylast {
() => {
// Module: crate::builtins::filters::array
// Provides: {"last"}
// Dependencies: {}
# [doc = " Returns the last value of an array"] # [doc = " If the array is empty, returns empty string"] pub fn last (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let mut arr = try_get_value ! ("last" , "value" , Vec < Value >, value) ; Ok (arr . pop () . unwrap_or_else (| | to_value ("") . unwrap ())) }
};
}
