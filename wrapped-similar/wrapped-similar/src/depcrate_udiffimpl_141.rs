// Generated macro for impl_141 (impl)
macro_rules! Depcrate_udiffimpl_141 {
() => {
// Module: crate::udiff
// Provides: {"impl_141"}
// Dependencies: {}
impl fmt :: Display for UnifiedDiffHunkRange { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut beginning = self . start () + 1 ; let len = self . end () . saturating_sub (self . start ()) ; if len == 1 { write ! (f , "{}" , beginning) } else { if len == 0 { beginning -= 1 ; } write ! (f , "{},{}" , beginning , len) } } }
};
}
