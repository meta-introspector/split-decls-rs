// Generated macro for impl_739 (impl)
macro_rules! Depcrate_util_boxed_cloneimpl_739 {
() => {
// Module: crate::util::boxed_clone
// Provides: {"impl_739"}
// Dependencies: {}
impl < R , T > CloneService < R > for T where T : Service < R > + Send + Clone + 'static , { fn clone_box (& self ,) -> Box < dyn CloneService < R , Response = T :: Response , Error = T :: Error , Future = T :: Future > + Send > { Box :: new (self . clone ()) } }
};
}
