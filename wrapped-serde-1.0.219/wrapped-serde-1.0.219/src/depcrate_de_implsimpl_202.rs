// Generated macro for impl_202 (impl)
macro_rules! Depcrate_de_implsimpl_202 {
() => {
// Module: crate::de::impls
// Provides: {"impl_202"}
// Dependencies: {}
impl < 'de : 'a , 'a > Deserialize < 'de > for & 'a [u8] { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_bytes (BytesVisitor) } }
};
}
