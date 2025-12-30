// Generated macro for impl_91 (impl)
macro_rules! Depcrate_framework_fmtimpl_91 {
() => {
// Module: crate::framework::fmt
// Provides: {"impl_91"}
// Dependencies: {}
impl < S , C > DebugWithContext < C > for MaybeReachable < S > where S : DebugWithContext < C > , { fn fmt_with (& self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { MaybeReachable :: Unreachable => { write ! (f , "unreachable") } MaybeReachable :: Reachable (set) => set . fmt_with (ctxt , f) , } } fn fmt_diff_with (& self , old : & Self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match (self , old) { (MaybeReachable :: Unreachable , MaybeReachable :: Unreachable) => Ok (()) , (MaybeReachable :: Unreachable , MaybeReachable :: Reachable (set)) => { write ! (f , "\u{001f}+") ? ; set . fmt_with (ctxt , f) } (MaybeReachable :: Reachable (set) , MaybeReachable :: Unreachable) => { write ! (f , "\u{001f}-") ? ; set . fmt_with (ctxt , f) } (MaybeReachable :: Reachable (this) , MaybeReachable :: Reachable (old)) => { this . fmt_diff_with (old , ctxt , f) } } } }
};
}
