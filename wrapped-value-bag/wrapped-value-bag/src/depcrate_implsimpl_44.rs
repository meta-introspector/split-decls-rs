// Generated macro for impl_44 (impl)
macro_rules! Depcrate_implsimpl_44 {
() => {
// Module: crate::impls
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'v > TryFrom < ValueBag < 'v > > for & 'v str { type Error = Error ; # [inline] fn try_from (v : ValueBag < 'v >) -> Result < Self , Error > { v . to_borrowed_str () . ok_or_else (| | Error :: msg ("conversion failed")) } }
};
}
