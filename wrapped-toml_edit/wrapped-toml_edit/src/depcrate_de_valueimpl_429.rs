// Generated macro for impl_429 (impl)
macro_rules! Depcrate_de_valueimpl_429 {
() => {
// Module: crate::de::value
// Provides: {"impl_429"}
// Dependencies: {}
impl serde_core :: de :: IntoDeserializer < '_ , Error > for crate :: Value { type Deserializer = ValueDeserializer ; fn into_deserializer (self) -> Self :: Deserializer { ValueDeserializer :: new (crate :: Item :: Value (self)) } }
};
}
