// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl Drop for RestoreEnv < '_ > { fn drop (& mut self) { for (var , value) in self . env . iter () { update_env (var , value . as_ref () . map (| v | v . as_os_str ())) ; } } }
};
}
