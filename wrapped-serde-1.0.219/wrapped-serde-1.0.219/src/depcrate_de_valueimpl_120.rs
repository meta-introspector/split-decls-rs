// Generated macro for impl_120 (impl)
macro_rules! Depcrate_de_valueimpl_120 {
() => {
// Module: crate::de::value
// Provides: {"impl_120"}
// Dependencies: {}
impl < 'de , A > IntoDeserializer < 'de , A :: Error > for SeqAccessDeserializer < A > where A : de :: SeqAccess < 'de > , { type Deserializer = Self ; fn into_deserializer (self) -> Self { self } }
};
}
