// Generated macro for impl_143 (impl)
macro_rules! Depcrate_bytebufimpl_143 {
() => {
// Module: crate::bytebuf
// Provides: {"impl_143"}
// Dependencies: {}
impl Serialize for ByteBuf { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_bytes (& self . bytes) } }
};
}
