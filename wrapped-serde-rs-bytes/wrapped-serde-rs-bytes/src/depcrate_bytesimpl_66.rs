// Generated macro for impl_66 (impl)
macro_rules! Depcrate_bytesimpl_66 {
() => {
// Module: crate::bytes
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'a , 'de : 'a > Deserialize < 'de > for & 'a Bytes { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Deserialize :: deserialize (deserializer) . map (Bytes :: new) } }
};
}
