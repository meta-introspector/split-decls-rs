// Generated macro for impl_221 (impl)
macro_rules! Depcrate_serdeimpl_221 {
() => {
// Module: crate::serde
// Provides: {"impl_221"}
// Dependencies: {}
impl < 'de , 'data , Store > Deserialize < 'de > for ZeroTrie < Store > where 'de : 'data , Store : From < & 'data [u8] > + From < Vec < u8 > > + 'data , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { if deserializer . is_human_readable () { let lm = LiteMap :: < Box < ByteStr > , usize > :: deserialize (deserializer) ? ; ZeroTrie :: < Vec < u8 > > :: try_from (& lm) . map_err (D :: Error :: custom) . map (| trie | trie . convert_store ()) } else { let bytes = < & [u8] > :: deserialize (deserializer) ? ; let (tag , trie_bytes) = bytes . split_first () . ok_or (D :: Error :: custom ("expected at least 1 byte for ZeroTrie")) ? ; let store = Store :: from (trie_bytes) ; let zerotrie = if * tag == ZeroTrieSimpleAscii :: < u8 > :: OPTIONS . to_u8_flags () { ZeroTrieSimpleAscii :: from_store (store) . into_zerotrie () } else if * tag == ZeroTriePerfectHash :: < u8 > :: OPTIONS . to_u8_flags () { ZeroTriePerfectHash :: from_store (store) . into_zerotrie () } else if * tag == ZeroTrieExtendedCapacity :: < u8 > :: OPTIONS . to_u8_flags () { ZeroTrieExtendedCapacity :: from_store (store) . into_zerotrie () } else { return Err (D :: Error :: custom ("invalid ZeroTrie tag")) ; } ; Ok (zerotrie) } } }
};
}
