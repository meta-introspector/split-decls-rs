// Generated macro for impl_22 (impl)
macro_rules! Depcrate_configureimpl_22 {
() => {
// Module: crate::configure
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'de , T > Deserialize < 'de > for Readable < T > where T : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { T :: deserialize (Readable (deserializer)) . map (Readable) } fn deserialize_in_place < D > (deserializer : D , place : & mut Self) -> Result < () , D :: Error > where D : Deserializer < 'de > , { T :: deserialize_in_place (Readable (deserializer) , & mut place . 0) } }
};
}
