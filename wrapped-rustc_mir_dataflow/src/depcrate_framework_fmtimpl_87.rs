// Generated macro for impl_87 (impl)
macro_rules! Depcrate_framework_fmtimpl_87 {
() => {
// Module: crate::framework::fmt
// Provides: {"impl_87"}
// Dependencies: {}
impl < T , C > fmt :: Debug for DebugDiffWithAdapter < '_ , T , C > where T : DebugWithContext < C > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . new . fmt_diff_with (& self . old , self . ctxt , f) } }
};
}
