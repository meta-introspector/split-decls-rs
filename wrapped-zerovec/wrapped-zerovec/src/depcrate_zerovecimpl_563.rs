// Generated macro for impl_563 (impl)
macro_rules! Depcrate_zerovecimpl_563 {
() => {
// Module: crate::zerovec
// Provides: {"impl_563"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T : AsULE > FromIterator < T > for ZeroVec < '_ , T > { # [doc = " Creates an owned [`ZeroVec`] from an iterator of values."] fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = T > , { ZeroVec :: new_owned (iter . into_iter () . map (| t | t . to_unaligned ()) . collect ()) } }
};
}
