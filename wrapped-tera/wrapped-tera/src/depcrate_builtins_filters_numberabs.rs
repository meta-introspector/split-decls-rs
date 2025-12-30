// Generated macro for abs (function)
macro_rules! Depcrate_builtins_filters_numberabs {
() => {
// Module: crate::builtins::filters::number
// Provides: {"abs"}
// Dependencies: {}
# [doc = " Returns the absolute value of the argument."] pub fn abs (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { if value . as_u64 () . is_some () { Ok (value . clone ()) } else if let Some (num) = value . as_i64 () { Ok (to_value (num . abs ()) . unwrap ()) } else if let Some (num) = value . as_f64 () { Ok (to_value (num . abs ()) . unwrap ()) } else { Err (Error :: msg ("Filter `abs` was used on a value that isn't a number.")) } }
};
}
