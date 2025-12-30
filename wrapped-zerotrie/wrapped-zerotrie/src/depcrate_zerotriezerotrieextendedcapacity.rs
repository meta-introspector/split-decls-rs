// Generated macro for ZeroTrieExtendedCapacity (struct)
macro_rules! Depcrate_zerotrieZeroTrieExtendedCapacity {
() => {
// Module: crate::zerotrie
// Provides: {"ZeroTrieExtendedCapacity"}
// Dependencies: {}
# [doc = " A data structure that maps from a large number of byte strings to integers."] # [doc = ""] # [doc = " For more information, see [`ZeroTrie`]."] # [repr (transparent)] # [derive (Debug , Default , Clone , Copy , PartialEq , Eq)] # [cfg_attr (feature = "databake" , derive (databake :: Bake))] # [cfg_attr (feature = "databake" , databake (path = zerotrie))] # [allow (clippy :: exhaustive_structs)] pub struct ZeroTrieExtendedCapacity < Store : ? Sized > { # [doc (hidden)] pub store : Store , }
};
}
