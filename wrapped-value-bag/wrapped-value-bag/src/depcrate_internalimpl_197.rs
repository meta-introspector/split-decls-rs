// Generated macro for impl_197 (impl)
macro_rules! Depcrate_internalimpl_197 {
() => {
// Module: crate::internal
// Provides: {"impl_197"}
// Dependencies: {}
impl < 'v > ValueBag < 'v > { # [doc = " Visit the value using an internal visitor."] # [inline] pub (crate) fn internal_visit (& self , visitor : impl InternalVisitor < 'v >) -> Result < () , Error > { self . inner . internal_visit (visitor) } }
};
}
