// Generated macro for impl_52 (impl)
macro_rules! Depcrate_serde_implsimpl_52 {
() => {
// Module: crate::serde_impls
// Provides: {"impl_52"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for TextSize { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { u32 :: deserialize (deserializer) . map (TextSize :: from) } }
};
}
