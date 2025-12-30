// Generated macro for serialize (function)
macro_rules! Depcrate_serde_iso8601serialize {
() => {
// Module: crate::serde::iso8601
// Provides: {"serialize"}
// Dependencies: {}
# [doc = " Serialize an [`OffsetDateTime`] using the well-known ISO 8601 format."] # [cfg (feature = "formatting")] # [inline] pub fn serialize < S : Serializer > (datetime : & OffsetDateTime , serializer : S ,) -> Result < S :: Ok , S :: Error > { datetime . format (& Iso8601 :: < SERDE_CONFIG >) . map_err (S :: Error :: custom) ? . serialize (serializer) }
};
}
