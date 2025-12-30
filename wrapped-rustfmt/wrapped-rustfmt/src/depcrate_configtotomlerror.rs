// Generated macro for ToTomlError (struct)
macro_rules! Depcrate_configToTomlError {
() => {
// Module: crate::config
// Provides: {"ToTomlError"}
// Dependencies: {}
# [derive (Error , Debug)] # [error ("Could not output config: {0}")] pub struct ToTomlError (toml :: ser :: Error) ;
};
}
