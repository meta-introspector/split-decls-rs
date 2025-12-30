// Generated macro for impl_37 (impl)
macro_rules! Depcrate_bytearrayimpl_37 {
() => {
// Module: crate::bytearray
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'a , 'de : 'a , const N : usize > Deserialize < 'de > for & 'a ByteArray < N > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_bytes (BorrowedByteArrayVisitor :: < N >) } }
};
}
