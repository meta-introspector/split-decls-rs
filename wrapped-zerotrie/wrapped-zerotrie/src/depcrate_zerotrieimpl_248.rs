// Generated macro for impl_248 (impl)
macro_rules! Depcrate_zerotrieimpl_248 {
() => {
// Module: crate::zerotrie
// Provides: {"impl_248"}
// Dependencies: {}
impl < Store > ZeroTrieSimpleAscii < Store > { # [doc = " Wrap this specific ZeroTrie variant into a ZeroTrie."] # [inline] pub const fn into_zerotrie (self) -> ZeroTrie < Store > { ZeroTrie (ZeroTrieFlavor :: SimpleAscii (self)) } }
};
}
