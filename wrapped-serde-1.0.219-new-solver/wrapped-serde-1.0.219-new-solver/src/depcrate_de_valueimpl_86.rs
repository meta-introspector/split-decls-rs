// Generated macro for impl_86 (impl)
macro_rules! Depcrate_de_valueimpl_86 {
() => {
// Module: crate::de::value
// Provides: {"impl_86"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl < 'de , 'a , E > IntoDeserializer < 'de , E > for Cow < 'a , str > where E : de :: Error , { type Deserializer = CowStrDeserializer < 'a , E > ; fn into_deserializer (self) -> CowStrDeserializer < 'a , E > { CowStrDeserializer :: new (self) } }
};
}
