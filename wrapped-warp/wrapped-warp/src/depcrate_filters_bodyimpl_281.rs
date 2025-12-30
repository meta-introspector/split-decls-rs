// Generated macro for impl_281 (impl)
macro_rules! Depcrate_filters_bodyimpl_281 {
() => {
// Module: crate::filters::body
// Provides: {"impl_281"}
// Dependencies: {}
impl Decode for Json { const MIME : (mime :: Name < 'static > , mime :: Name < 'static >) = (mime :: APPLICATION , mime :: JSON) ; const WITH_NO_CONTENT_TYPE : bool = true ; fn decode < B : Buf , T : DeserializeOwned > (mut buf : B) -> Result < T , BoxError > { serde_json :: from_slice (& buf . copy_to_bytes (buf . remaining ())) . map_err (Into :: into) } }
};
}
