// Generated macro for impl_205 (impl)
macro_rules! Depcrate_de_implsimpl_205 {
() => {
// Module: crate::de::impls
// Provides: {"impl_205"}
// Dependencies: {}
# [cfg (any (feature = "std" , all (not (no_core_cstr) , feature = "alloc")))] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl < 'de > Deserialize < 'de > for CString { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_byte_buf (CStringVisitor) } }
};
}
