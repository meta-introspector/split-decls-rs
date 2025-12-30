// Generated macro for impl_65 (impl)
macro_rules! Depcrate_bytesimpl_65 {
() => {
// Module: crate::bytes
// Provides: {"impl_65"}
// Dependencies: {}
impl Serialize for Bytes { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_bytes (& self . bytes) } }
};
}
