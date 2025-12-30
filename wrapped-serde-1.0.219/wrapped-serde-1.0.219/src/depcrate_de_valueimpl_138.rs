// Generated macro for impl_138 (impl)
macro_rules! Depcrate_de_valueimpl_138 {
() => {
// Module: crate::de::value
// Provides: {"impl_138"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl < 'de , K , V , S , E > IntoDeserializer < 'de , E > for HashMap < K , V , S > where K : IntoDeserializer < 'de , E > + Eq + Hash , V : IntoDeserializer < 'de , E > , S : BuildHasher , E : de :: Error , { type Deserializer = MapDeserializer < 'de , < Self as IntoIterator > :: IntoIter , E > ; fn into_deserializer (self) -> Self :: Deserializer { MapDeserializer :: new (self . into_iter ()) } }
};
}
