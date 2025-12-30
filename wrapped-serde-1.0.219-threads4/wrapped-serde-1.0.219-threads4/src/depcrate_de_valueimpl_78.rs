// Generated macro for impl_78 (impl)
macro_rules! Depcrate_de_valueimpl_78 {
() => {
// Module: crate::de::value
// Provides: {"impl_78"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl < 'de , E > IntoDeserializer < 'de , E > for String where E : de :: Error , { type Deserializer = StringDeserializer < E > ; fn into_deserializer (self) -> StringDeserializer < E > { StringDeserializer :: new (self) } }
};
}
