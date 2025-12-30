// Generated macro for impl_53 (impl)
macro_rules! Depcrate_algorithms_utilsimpl_53 {
() => {
// Module: crate::algorithms::utils
// Provides: {"impl_53"}
// Dependencies: {}
impl < Int > Index < usize > for OffsetLookup < Int > { type Output = Int ; # [inline (always)] fn index (& self , index : usize) -> & Self :: Output { & self . vec [index - self . offset] } }
};
}
