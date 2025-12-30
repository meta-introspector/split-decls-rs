// Generated macro for impl_44 (impl)
macro_rules! Depcrate_configimpl_44 {
() => {
// Module: crate::config
// Provides: {"impl_44"}
// Dependencies: {}
impl WitConfig { # [doc = " Returns the name of the \"runner\" world"] pub fn runner_world (& self) -> & str { self . runner . as_deref () . unwrap_or ("runner") } # [doc = " Returns the list of dependency worlds that this configuration uses."] pub fn dependency_worlds (& self) -> Vec < String > { match self . dependencies . clone () { Some (list) => list . into () , None => vec ! ["test" . to_string ()] , } } }
};
}
