// Generated macro for impl_256 (impl)
macro_rules! Depcrate_zerotrieimpl_256 {
() => {
// Module: crate::zerotrie
// Provides: {"impl_256"}
// Dependencies: {}
impl < Store > ZeroTrieExtendedCapacity < Store > { # [doc = " Wrap this specific ZeroTrie variant into a ZeroTrie."] # [inline] pub const fn into_zerotrie (self) -> ZeroTrie < Store > { ZeroTrie (ZeroTrieFlavor :: ExtendedCapacity (self)) } }
};
}
