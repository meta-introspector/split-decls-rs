// Generated macro for impl_107 (impl)
macro_rules! Depcrate_envimpl_107 {
() => {
// Module: crate::env
// Provides: {"impl_107"}
// Dependencies: {}
impl Update { pub fn env () -> Result < Self > { let Some (var) = env :: var_os ("TRYBUILD") else { return Ok (Update :: default ()) ; } ; match var . as_os_str () . to_str () { Some ("wip") => Ok (Update :: Wip) , Some ("overwrite") => Ok (Update :: Overwrite) , _ => Err (Error :: UpdateVar (var)) , } } }
};
}
