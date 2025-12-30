// Generated macro for undefined (function)
macro_rules! Depcrate_builtins_testersundefined {
() => {
// Module: crate::builtins::testers
// Provides: {"undefined"}
// Dependencies: {}
# [doc = " Returns true if `value` is undefined. Otherwise, returns false."] pub fn undefined (value : Option < & Value > , params : & [Value]) -> Result < bool > { number_args_allowed ("undefined" , 0 , params . len ()) ? ; Ok (value . is_none ()) }
};
}
