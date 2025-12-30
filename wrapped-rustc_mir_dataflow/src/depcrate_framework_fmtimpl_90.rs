// Generated macro for impl_90 (impl)
macro_rules! Depcrate_framework_fmtimpl_90 {
() => {
// Module: crate::framework::fmt
// Provides: {"impl_90"}
// Dependencies: {}
impl < T , C > DebugWithContext < C > for MixedBitSet < T > where T : Idx + DebugWithContext < C > , { fn fmt_with (& self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { MixedBitSet :: Small (set) => set . fmt_with (ctxt , f) , MixedBitSet :: Large (set) => set . fmt_with (ctxt , f) , } } fn fmt_diff_with (& self , old : & Self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match (self , old) { (MixedBitSet :: Small (set) , MixedBitSet :: Small (old)) => set . fmt_diff_with (old , ctxt , f) , (MixedBitSet :: Large (set) , MixedBitSet :: Large (old)) => set . fmt_diff_with (old , ctxt , f) , _ => panic ! ("MixedBitSet size mismatch") , } } }
};
}
