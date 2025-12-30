// Generated macro for impl_214 (impl)
macro_rules! Depcrate_serdeimpl_214 {
() => {
// Module: crate::serde
// Provides: {"impl_214"}
// Dependencies: {}
impl < Store > Serialize for ZeroTrieSimpleAscii < Store > where Store : AsRef < [u8] > , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if serializer . is_human_readable () { let lm = self . to_litemap () ; lm . serialize (serializer) } else { (Self :: FLAGS , ByteStr :: from_bytes (self . as_bytes ())) . serialize (serializer) } } }
};
}
