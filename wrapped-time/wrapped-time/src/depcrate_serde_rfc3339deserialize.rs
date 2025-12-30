// Generated macro for deserialize (function)
macro_rules! Depcrate_serde_rfc3339deserialize {
() => {
// Module: crate::serde::rfc3339
// Provides: {"deserialize"}
// Dependencies: {}
# [doc = " Deserialize an [`OffsetDateTime`] from its RFC3339 representation."] # [cfg (feature = "parsing")] # [inline] pub fn deserialize < 'a , D : Deserializer < 'a > > (deserializer : D) -> Result < OffsetDateTime , D :: Error > { deserializer . deserialize_str (Visitor :: < Rfc3339 > (PhantomData)) }
};
}
