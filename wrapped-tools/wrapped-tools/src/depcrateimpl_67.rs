// Generated macro for impl_67 (impl)
macro_rules! Depcrateimpl_67 {
() => {
// Module: crate
// Provides: {"impl_67"}
// Dependencies: {}
impl < 'a > Env < 'a > { # [doc = " Create a new instance."] pub fn new () -> Self { Env { altered_vars : Vec :: new () , } } # [doc = " Set `var` to `value`."] pub fn set (mut self , var : & 'a str , value : impl Into < String >) -> Self { let prev = env :: var_os (var) ; env :: set_var (var , value . into ()) ; self . altered_vars . push ((var , prev)) ; self } # [doc = " Unset `var`."] pub fn unset (mut self , var : & 'a str) -> Self { let prev = env :: var_os (var) ; env :: remove_var (var) ; self . altered_vars . push ((var , prev)) ; self } }
};
}
