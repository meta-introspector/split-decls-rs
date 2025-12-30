// Generated macro for impl_23 (impl)
macro_rules! Depcrate_configureimpl_23 {
() => {
// Module: crate::configure
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'de , T > Deserialize < 'de > for Compact < T > where T : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { T :: deserialize (Compact (deserializer)) . map (Compact) } fn deserialize_in_place < D > (deserializer : D , place : & mut Self) -> Result < () , D :: Error > where D : Deserializer < 'de > , { T :: deserialize_in_place (Compact (deserializer) , & mut place . 0) } }
};
}
