// Generated macro for impl_146 (impl)
macro_rules! Depcrate_bytebufimpl_146 {
() => {
// Module: crate::bytebuf
// Provides: {"impl_146"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for ByteBuf { fn deserialize < D > (deserializer : D) -> Result < ByteBuf , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_byte_buf (ByteBufVisitor) } }
};
}
