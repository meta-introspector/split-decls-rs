// Generated macro for impl_54 (impl)
macro_rules! Depcrate_builder_litemapimpl_54 {
() => {
// Module: crate::builder::litemap
// Provides: {"impl_54"}
// Dependencies: {}
impl < K , S > TryFrom < & LiteMap < K , usize , S > > for ZeroTrie < Vec < u8 > > where K : Borrow < [u8] > , S : litemap :: store :: StoreSlice < K , usize , Slice = [(K , usize)] > , { type Error = ZeroTrieBuildError ; fn try_from (items : & LiteMap < K , usize , S >) -> Result < Self , ZeroTrieBuildError > { let byte_litemap = items . to_borrowed_keys :: < [u8] , Vec < _ > > () ; let byte_slice = byte_litemap . as_slice () ; let byte_str_slice = ByteStr :: from_byte_slice_with_value (byte_slice) ; Self :: try_from_tuple_slice (byte_str_slice) } }
};
}
