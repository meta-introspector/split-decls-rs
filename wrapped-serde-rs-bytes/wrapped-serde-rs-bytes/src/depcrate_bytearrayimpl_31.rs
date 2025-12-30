// Generated macro for impl_31 (impl)
macro_rules! Depcrate_bytearrayimpl_31 {
() => {
// Module: crate::bytearray
// Provides: {"impl_31"}
// Dependencies: {}
impl < const N : usize > Serialize for ByteArray < N > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_bytes (& self . bytes) } }
};
}
