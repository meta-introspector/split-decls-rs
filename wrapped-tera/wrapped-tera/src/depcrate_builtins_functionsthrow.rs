// Generated macro for throw (function)
macro_rules! Depcrate_builtins_functionsthrow {
() => {
// Module: crate::builtins::functions
// Provides: {"throw"}
// Dependencies: {}
pub fn throw (args : & HashMap < String , Value >) -> Result < Value > { match args . get ("message") { Some (val) => match from_value :: < String > (val . clone ()) { Ok (v) => Err (Error :: msg (v)) , Err (_) => Err (Error :: msg (format ! ("Function `throw` received message={} but `message` can only be a string" , val))) , } , None => Err (Error :: msg ("Function `throw` was called without a `message` argument")) , } }
};
}
