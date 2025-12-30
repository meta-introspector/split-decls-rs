// Generated macro for impl_55 (impl)
macro_rules! Depcrate_de_valueimpl_55 {
() => {
// Module: crate::de::value
// Provides: {"impl_55"}
// Dependencies: {}
impl < 'de , E > IntoDeserializer < 'de , E > for u32 where E : de :: Error , { type Deserializer = U32Deserializer < E > ; fn into_deserializer (self) -> U32Deserializer < E > { U32Deserializer :: new (self) } }
};
}
