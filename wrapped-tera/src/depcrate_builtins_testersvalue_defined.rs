// Generated macro for value_defined (function)
macro_rules! Depcrate_builtins_testersvalue_defined {
() => {
// Module: crate::builtins::testers
// Provides: {"value_defined"}
// Dependencies: {}
# [doc = " Called to check if the Value is defined and return an Err if not"] pub fn value_defined (tester_name : & str , value : Option < & Value >) -> Result < () > { if value . is_none () { return Err (Error :: msg (format ! ("Tester `{}` was called on an undefined variable" , tester_name))) ; } Ok (()) }
};
}
