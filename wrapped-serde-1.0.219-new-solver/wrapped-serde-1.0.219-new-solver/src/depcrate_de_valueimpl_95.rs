// Generated macro for impl_95 (impl)
macro_rules! Depcrate_de_valueimpl_95 {
() => {
// Module: crate::de::value
// Provides: {"impl_95"}
// Dependencies: {}
impl < 'de , 'a , E > IntoDeserializer < 'de , E > for & 'a [u8] where E : de :: Error , { type Deserializer = BytesDeserializer < 'a , E > ; fn into_deserializer (self) -> BytesDeserializer < 'a , E > { BytesDeserializer :: new (self) } }
};
}
