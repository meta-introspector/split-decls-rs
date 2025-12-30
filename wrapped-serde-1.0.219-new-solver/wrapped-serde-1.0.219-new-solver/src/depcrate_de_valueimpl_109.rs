// Generated macro for impl_109 (impl)
macro_rules! Depcrate_de_valueimpl_109 {
() => {
// Module: crate::de::value
// Provides: {"impl_109"}
// Dependencies: {}
impl < 'de , I , T , E > IntoDeserializer < 'de , E > for SeqDeserializer < I , E > where I : Iterator < Item = T > , T : IntoDeserializer < 'de , E > , E : de :: Error , { type Deserializer = Self ; fn into_deserializer (self) -> Self { self } }
};
}
