// Generated macro for impl_253 (impl)
macro_rules! Depcrate_zerotrieimpl_253 {
() => {
// Module: crate::zerotrie
// Provides: {"impl_253"}
// Dependencies: {}
impl < Store > ZeroTriePerfectHash < Store > { # [doc = " Wrap this specific ZeroTrie variant into a ZeroTrie."] # [inline] pub const fn into_zerotrie (self) -> ZeroTrie < Store > { ZeroTrie (ZeroTrieFlavor :: PerfectHash (self)) } }
};
}
