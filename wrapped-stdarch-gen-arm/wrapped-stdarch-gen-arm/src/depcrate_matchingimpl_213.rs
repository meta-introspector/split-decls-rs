// Generated macro for impl_213 (impl)
macro_rules! Depcrate_matchingimpl_213 {
() => {
// Module: crate::matching
// Provides: {"impl_213"}
// Dependencies: {}
impl < T : fmt :: Debug > AsRef < T > for SizeMatchable < T > { fn as_ref (& self) -> & T { if let SizeMatchable :: Matched (v) = self { v } else { panic ! ("no match for {self:?} was performed") ; } } }
};
}
