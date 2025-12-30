// Generated macro for impl_265 (impl)
macro_rules! Depcrate_zerotrieimpl_265 {
() => {
// Module: crate::zerotrie
// Provides: {"impl_265"}
// Dependencies: {}
impl < Store > ZeroTrie < Store > { # [doc = " Takes the byte store from this trie."] pub fn into_store (self) -> Store { impl_dispatch ! (self , into_store ()) } # [doc = " Converts this trie's store to a different store implementing the `From` trait."] # [doc = ""] # [doc = " For example, use this to change `ZeroTrie<Vec<u8>>` to `ZeroTrie<Cow<[u8]>>`."] pub fn convert_store < NewStore > (self) -> ZeroTrie < NewStore > where NewStore : From < Store > , { impl_dispatch ! (self , convert_store () . into_zerotrie ()) } }
};
}
