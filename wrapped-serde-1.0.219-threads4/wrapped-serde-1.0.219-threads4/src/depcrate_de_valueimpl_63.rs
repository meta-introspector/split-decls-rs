// Generated macro for impl_63 (impl)
macro_rules! Depcrate_de_valueimpl_63 {
() => {
// Module: crate::de::value
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'de , 'a , E > IntoDeserializer < 'de , E > for & 'a str where E : de :: Error , { type Deserializer = StrDeserializer < 'a , E > ; fn into_deserializer (self) -> StrDeserializer < 'a , E > { StrDeserializer :: new (self) } }
};
}
