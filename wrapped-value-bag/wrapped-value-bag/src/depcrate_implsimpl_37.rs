// Generated macro for impl_37 (impl)
macro_rules! Depcrate_implsimpl_37 {
() => {
// Module: crate::impls
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'v > TryFrom < ValueBag < 'v > > for u128 { type Error = Error ; # [inline] fn try_from (v : ValueBag < 'v >) -> Result < Self , Error > { v . to_u128 () . ok_or_else (| | Error :: msg ("conversion failed")) } }
};
}
