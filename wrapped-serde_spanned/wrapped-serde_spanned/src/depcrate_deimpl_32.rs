// Generated macro for impl_32 (impl)
macro_rules! Depcrate_deimpl_32 {
() => {
// Module: crate::de
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'de , T , E > SpannedDeserializer < 'de , T , E > where T : serde_core :: de :: IntoDeserializer < 'de , E > , E : serde_core :: de :: Error , { # [doc = " Create a deserializer to emit [`Spanned`]"] pub fn new (value : T , span : core :: ops :: Range < usize >) -> Self { Self { start : Some (span . start) , end : Some (span . end) , value : Some (value) , _lifetime : Default :: default () , _error : Default :: default () , } } }
};
}
