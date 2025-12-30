// Generated macro for impl_196 (impl)
macro_rules! Depcrate_de_implsimpl_196 {
() => {
// Module: crate::de::impls
// Provides: {"impl_196"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl < 'de > Deserialize < 'de > for String { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_string (StringVisitor) } fn deserialize_in_place < D > (deserializer : D , place : & mut Self) -> Result < () , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_string (StringInPlaceVisitor (place)) } }
};
}
