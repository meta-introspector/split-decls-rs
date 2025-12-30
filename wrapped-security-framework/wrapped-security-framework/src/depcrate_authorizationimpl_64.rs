// Generated macro for impl_64 (impl)
macro_rules! Depcrate_authorizationimpl_64 {
() => {
// Module: crate::authorization
// Provides: {"impl_64"}
// Dependencies: {}
impl Drop for Authorization { # [inline] fn drop (& mut self) { unsafe { sys :: AuthorizationFree (self . handle , self . free_flags . bits ()) ; } } }
};
}
