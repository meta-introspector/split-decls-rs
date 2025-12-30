// Generated macro for impl_115 (impl)
macro_rules! Depcrate_de_valueimpl_115 {
() => {
// Module: crate::de::value
// Provides: {"impl_115"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl < 'de , T , E > IntoDeserializer < 'de , E > for BTreeSet < T > where T : IntoDeserializer < 'de , E > + Eq + Ord , E : de :: Error , { type Deserializer = SeqDeserializer < < Self as IntoIterator > :: IntoIter , E > ; fn into_deserializer (self) -> Self :: Deserializer { SeqDeserializer :: new (self . into_iter ()) } }
};
}
