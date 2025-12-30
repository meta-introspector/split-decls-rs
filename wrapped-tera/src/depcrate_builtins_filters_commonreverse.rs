// Generated macro for reverse (function)
macro_rules! Depcrate_builtins_filters_commonreverse {
() => {
// Module: crate::builtins::filters::common
// Provides: {"reverse"}
// Dependencies: {}
pub fn reverse (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { match value { Value :: Array (arr) => { let mut rev = arr . clone () ; rev . reverse () ; to_value (& rev) . map_err (Error :: json) } Value :: String (s) => to_value (String :: from_iter (s . chars () . rev ())) . map_err (Error :: json) , _ => Err (Error :: msg (format ! ("Filter `reverse` received an incorrect type for arg `value`: \
             got `{}` but expected Array|String" , value))) , } }
};
}
