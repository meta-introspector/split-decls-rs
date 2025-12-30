// Generated macro for impl_268 (impl)
macro_rules! Depcrate_zerotrieimpl_268 {
() => {
// Module: crate::zerotrie
// Provides: {"impl_268"}
// Dependencies: {}
# [cfg (feature = "litemap")] impl < Store > ZeroTrie < Store > where Store : AsRef < [u8] > , { # [doc = " Exports the data from this ZeroTrie into a LiteMap."] pub fn to_litemap (& self) -> LiteMap < Box < [u8] > , usize > { impl_dispatch ! (& self , to_litemap_bytes ()) } }
};
}
