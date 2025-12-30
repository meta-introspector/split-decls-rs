// Generated macro for defined (function)
macro_rules! Depcrate_builtins_testersdefined {
() => {
// Module: crate::builtins::testers
// Provides: {"defined"}
// Dependencies: {}
# [doc = " Returns true if `value` is defined. Otherwise, returns false."] pub fn defined (value : Option < & Value > , params : & [Value]) -> Result < bool > { number_args_allowed ("defined" , 0 , params . len ()) ? ; Ok (value . is_some ()) }
};
}
