// Generated macro for impl_252 (impl)
macro_rules! Depcrate_de_implsimpl_252 {
() => {
// Module: crate::de::impls
// Provides: {"impl_252"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl < 'de > Deserialize < 'de > for PathBuf { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_string (PathBufVisitor) } }
};
}
