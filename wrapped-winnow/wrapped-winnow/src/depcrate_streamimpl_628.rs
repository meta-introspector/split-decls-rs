// Generated macro for impl_628 (impl)
macro_rules! Depcrate_streamimpl_628 {
() => {
// Module: crate::stream
// Provides: {"impl_628"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < K > Accumulate < K > for BTreeSet < K > where K : core :: cmp :: Ord , { # [inline (always)] fn initial (_capacity : Option < usize >) -> Self { BTreeSet :: new () } # [inline (always)] fn accumulate (& mut self , key : K) { self . insert (key) ; } }
};
}
