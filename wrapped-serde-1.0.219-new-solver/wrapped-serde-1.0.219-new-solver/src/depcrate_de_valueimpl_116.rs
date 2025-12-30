// Generated macro for impl_116 (impl)
macro_rules! Depcrate_de_valueimpl_116 {
() => {
// Module: crate::de::value
// Provides: {"impl_116"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl < 'de , T , S , E > IntoDeserializer < 'de , E > for HashSet < T , S > where T : IntoDeserializer < 'de , E > + Eq + Hash , S : BuildHasher , E : de :: Error , { type Deserializer = SeqDeserializer < < Self as IntoIterator > :: IntoIter , E > ; fn into_deserializer (self) -> Self :: Deserializer { SeqDeserializer :: new (self . into_iter ()) } }
};
}
