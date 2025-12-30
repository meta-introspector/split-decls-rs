// Generated macro for impl_217 (impl)
macro_rules! Depcrate_serdeimpl_217 {
() => {
// Module: crate::serde
// Provides: {"impl_217"}
// Dependencies: {}
impl < 'de , 'data , Store > Deserialize < 'de > for ZeroTriePerfectHash < Store > where 'de : 'data , Store : From < & 'data [u8] > + From < Vec < u8 > > + 'data , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { if deserializer . is_human_readable () { let lm = LiteMap :: < Box < ByteStr > , usize > :: deserialize (deserializer) ? ; ZeroTriePerfectHash :: try_from_serde_litemap (& lm) . map_err (D :: Error :: custom) . map (| trie | trie . convert_store ()) } else { let (flags , trie_bytes) = < (u8 , & [u8]) > :: deserialize (deserializer) ? ; if Self :: OPTIONS . to_u8_flags () != flags { return Err (D :: Error :: custom ("invalid ZeroTrie tag")) ; } Ok (ZeroTriePerfectHash :: from_store (Store :: from (trie_bytes))) } } }
};
}
