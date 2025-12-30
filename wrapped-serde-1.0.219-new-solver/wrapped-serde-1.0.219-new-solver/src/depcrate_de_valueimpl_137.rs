// Generated macro for impl_137 (impl)
macro_rules! Depcrate_de_valueimpl_137 {
() => {
// Module: crate::de::value
// Provides: {"impl_137"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl < 'de , K , V , E > IntoDeserializer < 'de , E > for BTreeMap < K , V > where K : IntoDeserializer < 'de , E > + Eq + Ord , V : IntoDeserializer < 'de , E > , E : de :: Error , { type Deserializer = MapDeserializer < 'de , < Self as IntoIterator > :: IntoIter , E > ; fn into_deserializer (self) -> Self :: Deserializer { MapDeserializer :: new (self . into_iter ()) } }
};
}
