// Generated macro for divisible_by (function)
macro_rules! Depcrate_builtins_testersdivisible_by {
() => {
// Module: crate::builtins::testers
// Provides: {"divisible_by"}
// Dependencies: {}
# [doc = " Returns true if `value` is divisible by the first param. Otherwise, returns false."] pub fn divisible_by (value : Option < & Value > , params : & [Value]) -> Result < bool > { number_args_allowed ("divisibleby" , 1 , params . len ()) ? ; value_defined ("divisibleby" , value) ? ; match value . and_then (| v | v . to_number () . ok ()) { Some (val) => match params . first () . and_then (| v | v . to_number () . ok ()) { Some (p) => Ok (val % p == 0.0) , None => Err (Error :: msg ("Tester `divisibleby` was called with a parameter that isn't a number" ,)) , } , None => { Err (Error :: msg ("Tester `divisibleby` was called on a variable that isn't a number")) } } }
};
}
