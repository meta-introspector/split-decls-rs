// Generated macro for _var_os (function)
macro_rules! Depcrate_env_var_os {
() => {
// Module: crate::env
// Provides: {"_var_os"}
// Dependencies: {}
fn _var_os (key : & OsStr) -> Option < OsString > { env_imp :: getenv (key) }
};
}
