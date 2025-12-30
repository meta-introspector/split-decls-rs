// Generated macro for impl_114 (impl)
macro_rules! Depcrate_de_valueimpl_114 {
() => {
// Module: crate::de::value
// Provides: {"impl_114"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl < 'de , T , E > IntoDeserializer < 'de , E > for Vec < T > where T : IntoDeserializer < 'de , E > , E : de :: Error , { type Deserializer = SeqDeserializer < < Self as IntoIterator > :: IntoIter , E > ; fn into_deserializer (self) -> Self :: Deserializer { SeqDeserializer :: new (self . into_iter ()) } }
};
}
