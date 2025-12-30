// Generated macro for impl_35 (impl)
macro_rules! Depcrate_contentimpl_35 {
() => {
// Module: crate::content
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'de , E > IntoDeserializer < 'de , E > for Content < 'de > where E : de :: Error , { type Deserializer = ContentDeserializer < 'de , E > ; fn into_deserializer (self) -> Self :: Deserializer { ContentDeserializer :: new (self) } }
};
}
