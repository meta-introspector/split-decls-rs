// Generated macro for impl_89 (impl)
macro_rules! Depcrate_de_valueimpl_89 {
() => {
// Module: crate::de::value
// Provides: {"impl_89"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'de , 'a , E > IntoDeserializer < 'de , E > for CowStrDeserializer < 'a , E > where E : de :: Error , { type Deserializer = Self ; fn into_deserializer (self) -> Self { self } }
};
}
