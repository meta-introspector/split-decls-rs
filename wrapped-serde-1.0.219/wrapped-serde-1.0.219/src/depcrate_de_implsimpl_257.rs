// Generated macro for impl_257 (impl)
macro_rules! Depcrate_de_implsimpl_257 {
() => {
// Module: crate::de::impls
// Provides: {"impl_257"}
// Dependencies: {}
# [cfg (all (feature = "std" , any (unix , windows)))] # [cfg_attr (docsrs , doc (cfg (all (feature = "std" , any (unix , windows)))))] impl < 'de > Deserialize < 'de > for OsString { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_enum ("OsString" , OSSTR_VARIANTS , OsStringVisitor) } }
};
}
