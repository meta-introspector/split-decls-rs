// Generated macro for impl_83 (impl)
macro_rules! Depcrate_internal_errorimpl_83 {
() => {
// Module: crate::internal::error
// Provides: {"impl_83"}
// Dependencies: {}
impl < 'v > TryFrom < ValueBag < 'v > > for & 'v (dyn error :: Error + 'static) { type Error = crate :: Error ; # [inline] fn try_from (v : ValueBag < 'v >) -> Result < Self , Self :: Error > { v . to_borrowed_error () . ok_or_else (| | Self :: Error :: msg ("conversion failed")) } }
};
}
