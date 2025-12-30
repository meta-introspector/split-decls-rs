// Generated macro for impl_218 (impl)
macro_rules! Depcrate_matchingimpl_218 {
() => {
// Module: crate::matching
// Provides: {"impl_218"}
// Dependencies: {}
impl < T : fmt :: Debug > AsRef < T > for KindMatchable < T > { fn as_ref (& self) -> & T { if let KindMatchable :: Matched (v) = self { v } else { panic ! ("no match for {self:?} was performed") ; } } }
};
}
