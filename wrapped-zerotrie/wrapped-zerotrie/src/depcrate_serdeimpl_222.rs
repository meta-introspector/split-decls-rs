// Generated macro for impl_222 (impl)
macro_rules! Depcrate_serdeimpl_222 {
() => {
// Module: crate::serde
// Provides: {"impl_222"}
// Dependencies: {}
impl < Store > Serialize for ZeroTrie < Store > where Store : AsRef < [u8] > , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if serializer . is_human_readable () { let lm = self . to_litemap () ; let lm = lm . iter () . map (| (k , v) | (ByteStr :: from_bytes (k) , v)) . collect :: < LiteMap < _ , _ > > () ; lm . serialize (serializer) } else { let (tag , bytes) = match & self . 0 { ZeroTrieFlavor :: SimpleAscii (t) => (ZeroTrieSimpleAscii :: < u8 > :: OPTIONS . to_u8_flags () , t . as_bytes () ,) , ZeroTrieFlavor :: PerfectHash (t) => (ZeroTriePerfectHash :: < u8 > :: OPTIONS . to_u8_flags () , t . as_bytes () ,) , ZeroTrieFlavor :: ExtendedCapacity (t) => (ZeroTrieExtendedCapacity :: < u8 > :: OPTIONS . to_u8_flags () , t . as_bytes () ,) , } ; let mut all_in_one_vec = Vec :: with_capacity (bytes . len () + 1) ; all_in_one_vec . push (tag) ; all_in_one_vec . extend (bytes) ; serializer . serialize_bytes (& all_in_one_vec) } } }
};
}
