// Generated macro for option (module)
macro_rules! Depcrate_serde_iso8601option {
() => {
// Module: crate::serde::iso8601
// Provides: {"option"}
// Dependencies: {}
# [doc = " Use the well-known ISO 8601 format when serializing and deserializing an"] # [doc = " [`Option<OffsetDateTime>`]."] # [doc = ""] # [doc = " Use this module in combination with serde's [`#[with]`][with] attribute."] # [doc = ""] # [doc = " [ISO 8601 format]: https://www.iso.org/iso-8601-date-and-time-format.html"] # [doc = " [with]: https://serde.rs/field-attrs.html#with"] pub mod option { use super :: * ; # [doc = " Serialize an [`Option<OffsetDateTime>`] using the well-known ISO 8601 format."] # [cfg (feature = "formatting")] # [inline] pub fn serialize < S : Serializer > (option : & Option < OffsetDateTime > , serializer : S ,) -> Result < S :: Ok , S :: Error > { option . map (| odt | odt . format (& Iso8601 :: < SERDE_CONFIG >)) . transpose () . map_err (S :: Error :: custom) ? . serialize (serializer) } # [doc = " Deserialize an [`Option<OffsetDateTime>`] from its ISO 8601 representation."] # [cfg (feature = "parsing")] # [inline] pub fn deserialize < 'a , D : Deserializer < 'a > > (deserializer : D ,) -> Result < Option < OffsetDateTime > , D :: Error > { deserializer . deserialize_option (Visitor :: < Option < Iso8601 < SERDE_CONFIG > > > (PhantomData)) } }
};
}
