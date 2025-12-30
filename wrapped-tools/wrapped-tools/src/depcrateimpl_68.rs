// Generated macro for impl_68 (impl)
macro_rules! Depcrateimpl_68 {
() => {
// Module: crate
// Provides: {"impl_68"}
// Dependencies: {}
impl Drop for Env < '_ > { fn drop (& mut self) { for (var , prev_value) in self . altered_vars . iter () . rev () { match prev_value { Some (value) => env :: set_var (var , value) , None => env :: remove_var (var) , } } } }
};
}
