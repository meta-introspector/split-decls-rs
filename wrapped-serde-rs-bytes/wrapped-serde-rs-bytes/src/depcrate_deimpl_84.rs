// Generated macro for impl_84 (impl)
macro_rules! Depcrate_deimpl_84 {
() => {
// Module: crate::de
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'de : 'a , 'a > Deserialize < 'de > for & 'a Bytes { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde :: Deserialize :: deserialize (deserializer) . map (Bytes :: new) } }
};
}
