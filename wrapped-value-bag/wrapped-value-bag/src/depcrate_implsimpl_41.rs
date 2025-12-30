// Generated macro for impl_41 (impl)
macro_rules! Depcrate_implsimpl_41 {
() => {
// Module: crate::impls
// Provides: {"impl_41"}
// Dependencies: {}
impl < 'v > TryFrom < ValueBag < 'v > > for i128 { type Error = Error ; # [inline] fn try_from (v : ValueBag < 'v >) -> Result < Self , Error > { v . to_i128 () . ok_or_else (| | Error :: msg ("conversion failed")) } }
};
}
