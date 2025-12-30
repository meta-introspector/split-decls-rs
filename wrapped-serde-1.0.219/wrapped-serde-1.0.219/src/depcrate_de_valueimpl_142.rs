// Generated macro for impl_142 (impl)
macro_rules! Depcrate_de_valueimpl_142 {
() => {
// Module: crate::de::value
// Provides: {"impl_142"}
// Dependencies: {}
impl < 'de , A > IntoDeserializer < 'de , A :: Error > for MapAccessDeserializer < A > where A : de :: MapAccess < 'de > , { type Deserializer = Self ; fn into_deserializer (self) -> Self { self } }
};
}
