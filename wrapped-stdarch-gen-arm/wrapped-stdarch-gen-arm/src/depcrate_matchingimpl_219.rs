// Generated macro for impl_219 (impl)
macro_rules! Depcrate_matchingimpl_219 {
() => {
// Module: crate::matching
// Provides: {"impl_219"}
// Dependencies: {}
impl < T : fmt :: Debug > AsMut < T > for KindMatchable < T > { fn as_mut (& mut self) -> & mut T { if let KindMatchable :: Matched (v) = self { v } else { panic ! ("no match for {self:?} was performed") ; } } }
};
}
