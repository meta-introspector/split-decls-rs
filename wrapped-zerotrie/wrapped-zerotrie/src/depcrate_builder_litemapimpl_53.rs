// Generated macro for impl_53 (impl)
macro_rules! Depcrate_builder_litemapimpl_53 {
() => {
// Module: crate::builder::litemap
// Provides: {"impl_53"}
// Dependencies: {}
impl ZeroTrieSimpleAscii < Vec < u8 > > { # [doc (hidden)] pub fn try_from_litemap_with_const_builder < 'a , S > (items : & LiteMap < & 'a [u8] , usize , S > ,) -> Result < Self , ZeroTrieBuildError > where S : litemap :: store :: StoreSlice < & 'a [u8] , usize , Slice = [(& 'a [u8] , usize)] > , { let tuples = items . as_slice () ; let byte_str_slice = ByteStr :: from_byte_slice_with_value (tuples) ; ZeroTrieBuilderConst :: < 10000 > :: from_sorted_const_tuple_slice :: < 100 > (byte_str_slice . into ()) . map (| s | Self { store : s . as_bytes () . to_vec () , }) } }
};
}
