// Generated macro for impl_26 (impl)
macro_rules! Depcrate_de_valueimpl_26 {
() => {
// Module: crate::de::value
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'de , E > IntoDeserializer < 'de , E > for () where E : de :: Error , { type Deserializer = UnitDeserializer < E > ; fn into_deserializer (self) -> UnitDeserializer < E > { UnitDeserializer :: new () } }
};
}
