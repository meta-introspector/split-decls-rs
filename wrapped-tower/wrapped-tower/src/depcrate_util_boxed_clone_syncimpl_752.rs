// Generated macro for impl_752 (impl)
macro_rules! Depcrate_util_boxed_clone_syncimpl_752 {
() => {
// Module: crate::util::boxed_clone_sync
// Provides: {"impl_752"}
// Dependencies: {}
impl < R , T > CloneService < R > for T where T : Service < R > + Send + Sync + Clone + 'static , { fn clone_box (& self ,) -> Box < dyn CloneService < R , Response = T :: Response , Error = T :: Error , Future = T :: Future > + Send + Sync , > { Box :: new (self . clone ()) } }
};
}
