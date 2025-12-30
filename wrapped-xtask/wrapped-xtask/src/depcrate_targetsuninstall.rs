// Generated macro for uninstall (function)
macro_rules! Depcrate_targetsuninstall {
() => {
// Module: crate::targets
// Provides: {"uninstall"}
// Dependencies: {}
pub fn uninstall (targets : Vec < String >) { println ! ("⏳ uninstalling targets") ; let status = Command :: new ("rustup") . args (["target" , "remove"]) . args (& targets) . status () . unwrap () ; if ! status . success () { eprintln ! ("Error uninstalling targets: {}" , targets . join (" ")) ; } }
};
}
