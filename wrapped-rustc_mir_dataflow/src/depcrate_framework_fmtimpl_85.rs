// Generated macro for impl_85 (impl)
macro_rules! Depcrate_framework_fmtimpl_85 {
() => {
// Module: crate::framework::fmt
// Provides: {"impl_85"}
// Dependencies: {}
impl < T , C > fmt :: Debug for DebugWithAdapter < '_ , T , C > where T : DebugWithContext < C > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . this . fmt_with (self . ctxt , f) } }
};
}
