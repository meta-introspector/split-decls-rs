// Generated macro for containing (function)
macro_rules! Depcrate_builtins_testerscontaining {
() => {
// Module: crate::builtins::testers
// Provides: {"containing"}
// Dependencies: {}
# [doc = " Returns true if `value` contains the given argument. Otherwise, returns false."] pub fn containing (value : Option < & Value > , params : & [Value]) -> Result < bool > { number_args_allowed ("containing" , 1 , params . len ()) ? ; value_defined ("containing" , value) ? ; match value . unwrap () { Value :: String (v) => { let needle = extract_string ("containing" , "with a parameter" , params . first ()) ? ; Ok (v . contains (needle)) } Value :: Array (v) => Ok (v . contains (params . first () . unwrap ())) , Value :: Object (v) => { let needle = extract_string ("containing" , "with a parameter" , params . first ()) ? ; Ok (v . contains_key (needle)) } _ => Err (Error :: msg ("Tester `containing` can only be used on string, array or map")) , } }
};
}
