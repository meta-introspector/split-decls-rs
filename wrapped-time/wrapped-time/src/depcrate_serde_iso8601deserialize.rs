// Generated macro for deserialize (function)
macro_rules! Depcrate_serde_iso8601deserialize {
() => {
// Module: crate::serde::iso8601
// Provides: {"deserialize"}
// Dependencies: {}
# [doc = " Deserialize an [`OffsetDateTime`] from its ISO 8601 representation."] # [cfg (feature = "parsing")] # [inline] pub fn deserialize < 'a , D : Deserializer < 'a > > (deserializer : D) -> Result < OffsetDateTime , D :: Error > { deserializer . deserialize_str (Visitor :: < Iso8601 < SERDE_CONFIG > > (PhantomData)) }
};
}
