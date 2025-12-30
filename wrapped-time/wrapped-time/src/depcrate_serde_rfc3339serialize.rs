// Generated macro for serialize (function)
macro_rules! Depcrate_serde_rfc3339serialize {
() => {
// Module: crate::serde::rfc3339
// Provides: {"serialize"}
// Dependencies: {}
# [doc = " Serialize an [`OffsetDateTime`] using the well-known RFC3339 format."] # [cfg (feature = "formatting")] # [inline] pub fn serialize < S : Serializer > (datetime : & OffsetDateTime , serializer : S ,) -> Result < S :: Ok , S :: Error > { datetime . format (& Rfc3339) . map_err (S :: Error :: custom) ? . serialize (serializer) }
};
}
