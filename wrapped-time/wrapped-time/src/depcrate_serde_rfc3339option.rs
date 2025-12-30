// Generated macro for option (module)
macro_rules! Depcrate_serde_rfc3339option {
() => {
// Module: crate::serde::rfc3339
// Provides: {"option"}
// Dependencies: {}
# [doc = " Use the well-known [RFC3339 format] when serializing and deserializing an"] # [doc = " [`Option<OffsetDateTime>`]."] # [doc = ""] # [doc = " Use this module in combination with serde's [`#[with]`][with] attribute."] # [doc = ""] # [doc = " [RFC3339 format]: https://tools.ietf.org/html/rfc3339#section-5.6"] # [doc = " [with]: https://serde.rs/field-attrs.html#with"] pub mod option { use super :: * ; # [doc = " Serialize an [`Option<OffsetDateTime>`] using the well-known RFC3339 format."] # [cfg (feature = "formatting")] # [inline] pub fn serialize < S : Serializer > (option : & Option < OffsetDateTime > , serializer : S ,) -> Result < S :: Ok , S :: Error > { option . map (| odt | odt . format (& Rfc3339)) . transpose () . map_err (S :: Error :: custom) ? . serialize (serializer) } # [doc = " Deserialize an [`Option<OffsetDateTime>`] from its RFC3339 representation."] # [cfg (feature = "parsing")] # [inline] pub fn deserialize < 'a , D : Deserializer < 'a > > (deserializer : D ,) -> Result < Option < OffsetDateTime > , D :: Error > { deserializer . deserialize_option (Visitor :: < Option < Rfc3339 > > (PhantomData)) } }
};
}
