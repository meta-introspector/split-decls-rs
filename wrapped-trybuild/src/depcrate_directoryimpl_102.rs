// Generated macro for impl_102 (impl)
macro_rules! Depcrate_directoryimpl_102 {
() => {
// Module: crate::directory
// Provides: {"impl_102"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Directory { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { PathBuf :: deserialize (deserializer) . map (Directory :: new) } }
};
}
