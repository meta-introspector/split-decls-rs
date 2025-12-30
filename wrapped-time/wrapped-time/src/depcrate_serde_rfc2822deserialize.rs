// Generated macro for deserialize (function)
macro_rules! Depcrate_serde_rfc2822deserialize {
() => {
// Module: crate::serde::rfc2822
// Provides: {"deserialize"}
// Dependencies: {}
# [doc = " Deserialize an [`OffsetDateTime`] from its RFC2822 representation."] # [cfg (feature = "parsing")] # [inline] pub fn deserialize < 'a , D : Deserializer < 'a > > (deserializer : D) -> Result < OffsetDateTime , D :: Error > { deserializer . deserialize_str (Visitor :: < Rfc2822 > (PhantomData)) }
};
}
