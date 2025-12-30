// Generated macro for _var (function)
macro_rules! Depcrate_env_var {
() => {
// Module: crate::env
// Provides: {"_var"}
// Dependencies: {}
fn _var (key : & OsStr) -> Result < String , VarError > { match var_os (key) { Some (s) => s . into_string () . map_err (VarError :: NotUnicode) , None => Err (VarError :: NotPresent) , } }
};
}
