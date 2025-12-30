// Generated macro for impl_220 (impl)
macro_rules! Depcrate_serdeimpl_220 {
() => {
// Module: crate::serde
// Provides: {"impl_220"}
// Dependencies: {}
impl < Store > Serialize for ZeroTrieExtendedCapacity < Store > where Store : AsRef < [u8] > , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if serializer . is_human_readable () { let lm = self . to_litemap () ; let lm = lm . iter () . map (| (k , v) | (ByteStr :: from_bytes (k) , v)) . collect :: < LiteMap < _ , _ > > () ; lm . serialize (serializer) } else { (Self :: OPTIONS . to_u8_flags () , ByteStr :: from_bytes (self . as_bytes ()) ,) . serialize (serializer) } } }
};
}
