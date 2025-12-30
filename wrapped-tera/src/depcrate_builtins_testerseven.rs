// Generated macro for even (function)
macro_rules! Depcrate_builtins_testerseven {
() => {
// Module: crate::builtins::testers
// Provides: {"even"}
// Dependencies: {}
# [doc = " Returns true if `value` is an even number. Otherwise, returns false."] pub fn even (value : Option < & Value > , params : & [Value]) -> Result < bool > { number_args_allowed ("even" , 0 , params . len ()) ? ; value_defined ("even" , value) ? ; let is_odd = odd (value , params) ? ; Ok (! is_odd) }
};
}
