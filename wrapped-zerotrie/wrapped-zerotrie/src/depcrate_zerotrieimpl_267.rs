// Generated macro for impl_267 (impl)
macro_rules! Depcrate_zerotrieimpl_267 {
() => {
// Module: crate::zerotrie
// Provides: {"impl_267"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < Store > ZeroTrie < Store > where Store : AsRef < [u8] > , { # [doc = " Exports the data from this ZeroTrie into a BTreeMap."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] pub fn to_btreemap (& self) -> BTreeMap < Box < [u8] > , usize > { impl_dispatch ! (& self , to_btreemap_bytes ()) } }
};
}
