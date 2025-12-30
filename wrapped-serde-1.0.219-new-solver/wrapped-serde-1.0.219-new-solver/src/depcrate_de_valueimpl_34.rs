// Generated macro for impl_34 (impl)
macro_rules! Depcrate_de_valueimpl_34 {
() => {
// Module: crate::de::value
// Provides: {"impl_34"}
// Dependencies: {}
# [cfg (feature = "unstable")] # [cfg_attr (docsrs , doc (cfg (feature = "unstable")))] impl < 'de , E > IntoDeserializer < 'de , E > for ! where E : de :: Error , { type Deserializer = NeverDeserializer < E > ; fn into_deserializer (self) -> Self :: Deserializer { self } }
};
}
