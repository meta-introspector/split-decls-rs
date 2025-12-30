// Generated macro for iterable (function)
macro_rules! Depcrate_builtins_testersiterable {
() => {
// Module: crate::builtins::testers
// Provides: {"iterable"}
// Dependencies: {}
# [doc = " Returns true if `value` can be iterated over in Tera (ie is an array/tuple or an object)."] # [doc = " Otherwise, returns false."] pub fn iterable (value : Option < & Value > , params : & [Value]) -> Result < bool > { number_args_allowed ("iterable" , 0 , params . len ()) ? ; value_defined ("iterable" , value) ? ; Ok (value . unwrap () . is_array () || value . unwrap () . is_object ()) }
};
}
