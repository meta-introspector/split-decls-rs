// Generated macro for impl_34 (impl)
macro_rules! Depcrate_bytearrayimpl_34 {
() => {
// Module: crate::bytearray
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'de , const N : usize > Deserialize < 'de > for ByteArray < N > { fn deserialize < D > (deserializer : D) -> Result < ByteArray < N > , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_bytes (ByteArrayVisitor :: < N >) } }
};
}
