// Generated macro for impl_249 (impl)
macro_rules! Depcrate_de_implsimpl_249 {
() => {
// Module: crate::de::impls
// Provides: {"impl_249"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl < 'de : 'a , 'a > Deserialize < 'de > for & 'a Path { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_str (PathVisitor) } }
};
}
