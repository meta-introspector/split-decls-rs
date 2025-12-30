// Generated macro for impl_283 (impl)
macro_rules! Depcrate_de_implsimpl_283 {
() => {
// Module: crate::de::impls
// Provides: {"impl_283"}
// Dependencies: {}
impl < 'de , T > Deserialize < 'de > for Wrapping < T > where T : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Deserialize :: deserialize (deserializer) . map (Wrapping) } }
};
}
