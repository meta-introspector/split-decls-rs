// Generated macro for impl_430 (impl)
macro_rules! Depcrate_collatorimpl_430 {
() => {
// Module: crate::collator
// Provides: {"impl_430"}
// Dependencies: {}
impl TryInto < CollationData < 'static > > for & collator_serde :: CollationData { type Error = DataError ; fn try_into (self) -> Result < CollationData < 'static > , Self :: Error > { Ok (CollationData { trie : CodePointTrie :: < u32 > :: try_from (& self . trie) . map_err (| e | DataError :: custom ("trie conversion") . with_display_context (& e)) ? , contexts : ZeroVec :: alloc_from_slice (& self . contexts) , ce32s : ZeroVec :: alloc_from_slice (& self . ce32s) , ces : self . ces . iter () . map (| i | * i as u64) . collect () , }) } }
};
}
