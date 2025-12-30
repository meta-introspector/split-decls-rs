// Generated macro for get_env (function)
macro_rules! Depcrate_builtins_functionsget_env {
() => {
// Module: crate::builtins::functions
// Provides: {"get_env"}
// Dependencies: {}
pub fn get_env (args : & HashMap < String , Value >) -> Result < Value > { let name = match args . get ("name") { Some (val) => match from_value :: < String > (val . clone ()) { Ok (v) => v , Err (_) => { return Err (Error :: msg (format ! ("Function `get_env` received name={} but `name` can only be a string" , val))) ; } } , None => return Err (Error :: msg ("Function `get_env` didn't receive a `name` argument")) , } ; match std :: env :: var (& name) . ok () { Some (res) => Ok (Value :: String (res)) , None => match args . get ("default") { Some (default) => Ok (default . clone ()) , None => Err (Error :: msg (format ! ("Environment variable `{}` not found" , & name))) , } , } }
};
}
