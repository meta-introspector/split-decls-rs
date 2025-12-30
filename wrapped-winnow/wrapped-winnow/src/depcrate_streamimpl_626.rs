// Generated macro for impl_626 (impl)
macro_rules! Depcrate_streamimpl_626 {
() => {
// Module: crate::stream
// Provides: {"impl_626"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < K , V > Accumulate < (K , V) > for BTreeMap < K , V > where K : core :: cmp :: Ord , { # [inline (always)] fn initial (_capacity : Option < usize >) -> Self { BTreeMap :: new () } # [inline (always)] fn accumulate (& mut self , (key , value) : (K , V)) { self . insert (key , value) ; } }
};
}
