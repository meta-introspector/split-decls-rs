// Generated macro for serialize (function)
macro_rules! Depcrate_serde_rfc2822serialize {
() => {
// Module: crate::serde::rfc2822
// Provides: {"serialize"}
// Dependencies: {}
# [doc = " Serialize an [`OffsetDateTime`] using the well-known RFC2822 format."] # [cfg (feature = "formatting")] # [inline] pub fn serialize < S : Serializer > (datetime : & OffsetDateTime , serializer : S ,) -> Result < S :: Ok , S :: Error > { datetime . format (& Rfc2822) . map_err (S :: Error :: custom) ? . serialize (serializer) }
};
}
