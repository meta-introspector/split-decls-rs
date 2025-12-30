// Generated macro for impl_81 (impl)
macro_rules! Depcrate_de_valueimpl_81 {
() => {
// Module: crate::de::value
// Provides: {"impl_81"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'de , E > IntoDeserializer < 'de , E > for StringDeserializer < E > where E : de :: Error , { type Deserializer = Self ; fn into_deserializer (self) -> Self { self } }
};
}
