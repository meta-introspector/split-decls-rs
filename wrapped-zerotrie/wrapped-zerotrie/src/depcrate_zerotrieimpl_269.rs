// Generated macro for impl_269 (impl)
macro_rules! Depcrate_zerotrieimpl_269 {
() => {
// Module: crate::zerotrie
// Provides: {"impl_269"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl ZeroTrie < Vec < u8 > > { pub (crate) fn try_from_tuple_slice (items : & [(& ByteStr , usize)] ,) -> Result < Self , ZeroTrieBuildError > { let is_all_ascii = items . iter () . all (| (s , _) | s . is_all_ascii ()) ; if is_all_ascii && items . len () < 512 { ZeroTrieSimpleAscii :: try_from_tuple_slice (items) . map (| x | x . into_zerotrie ()) } else { ZeroTriePerfectHash :: try_from_tuple_slice (items) . map (| x | x . into_zerotrie ()) } } }
};
}
