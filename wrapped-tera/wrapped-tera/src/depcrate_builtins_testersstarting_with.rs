// Generated macro for starting_with (function)
macro_rules! Depcrate_builtins_testersstarting_with {
() => {
// Module: crate::builtins::testers
// Provides: {"starting_with"}
// Dependencies: {}
# [doc = " Returns true if `value` starts with the given string. Otherwise, returns false."] pub fn starting_with (value : Option < & Value > , params : & [Value]) -> Result < bool > { number_args_allowed ("starting_with" , 1 , params . len ()) ? ; value_defined ("starting_with" , value) ? ; let value = extract_string ("starting_with" , "on a variable" , value) ? ; let needle = extract_string ("starting_with" , "with a parameter" , params . first ()) ? ; Ok (value . starts_with (needle)) }
};
}
