// Generated macro for impl_270 (impl)
macro_rules! Depcrate_zerotrieimpl_270 {
() => {
// Module: crate::zerotrie
// Provides: {"impl_270"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < K > FromIterator < (K , usize) > for ZeroTrie < Vec < u8 > > where K : AsRef < [u8] > , { fn from_iter < T : IntoIterator < Item = (K , usize) > > (iter : T) -> Self { let items = Vec :: from_iter (iter) ; let mut items : Vec < (& [u8] , usize) > = items . iter () . map (| (k , v) | (k . as_ref () , * v)) . collect () ; items . sort () ; let byte_str_slice = ByteStr :: from_byte_slice_with_value (& items) ; # [expect (clippy :: unwrap_used)] Self :: try_from_tuple_slice (byte_str_slice) . unwrap () } }
};
}
