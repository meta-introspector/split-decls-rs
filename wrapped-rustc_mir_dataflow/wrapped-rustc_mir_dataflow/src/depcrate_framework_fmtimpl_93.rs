// Generated macro for impl_93 (impl)
macro_rules! Depcrate_framework_fmtimpl_93 {
() => {
// Module: crate::framework::fmt
// Provides: {"impl_93"}
// Dependencies: {}
impl < T , C > DebugWithContext < C > for & '_ T where T : DebugWithContext < C > , { fn fmt_with (& self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* self) . fmt_with (ctxt , f) } fn fmt_diff_with (& self , old : & Self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* self) . fmt_diff_with (* old , ctxt , f) } }
};
}
