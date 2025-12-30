// Generated macro for length (function)
macro_rules! Depcrate_builtins_filters_commonlength {
() => {
// Module: crate::builtins::filters::common
// Provides: {"length"}
// Dependencies: {}
pub fn length (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { match value { Value :: Array (arr) => Ok (to_value (arr . len ()) . unwrap ()) , Value :: Object (m) => Ok (to_value (m . len ()) . unwrap ()) , Value :: String (s) => Ok (to_value (s . chars () . count ()) . unwrap ()) , _ => Err (Error :: msg ("Filter `length` was used on a value that isn't an array, an object, or a string." ,)) , } }
};
}
