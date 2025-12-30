// Generated macro for first (function)
macro_rules! Depcrate_builtins_filters_arrayfirst {
() => {
// Module: crate::builtins::filters::array
// Provides: {"first"}
// Dependencies: {}
# [doc = " Returns the first value of an array"] # [doc = " If the array is empty, returns empty string"] pub fn first (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let mut arr = try_get_value ! ("first" , "value" , Vec < Value >, value) ; if arr . is_empty () { Ok (to_value ("") . unwrap ()) } else { Ok (arr . swap_remove (0)) } }
};
}
