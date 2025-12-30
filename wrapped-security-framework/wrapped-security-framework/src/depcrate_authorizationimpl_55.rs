// Generated macro for impl_55 (impl)
macro_rules! Depcrate_authorizationimpl_55 {
() => {
// Module: crate::authorization
// Provides: {"impl_55"}
// Dependencies: {}
impl Drop for AuthorizationItemSet < '_ > { # [inline] fn drop (& mut self) { unsafe { sys :: AuthorizationFreeItemSet (self . inner . cast_mut ()) ; } } }
};
}
