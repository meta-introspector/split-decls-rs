// Generated macro for impl_214 (impl)
macro_rules! Depcrate_matchingimpl_214 {
() => {
// Module: crate::matching
// Provides: {"impl_214"}
// Dependencies: {}
impl < T : fmt :: Debug > AsMut < T > for SizeMatchable < T > { fn as_mut (& mut self) -> & mut T { if let SizeMatchable :: Matched (v) = self { v } else { panic ! ("no match for {self:?} was performed") ; } } }
};
}
