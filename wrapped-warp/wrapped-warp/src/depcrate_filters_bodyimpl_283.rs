// Generated macro for impl_283 (impl)
macro_rules! Depcrate_filters_bodyimpl_283 {
() => {
// Module: crate::filters::body
// Provides: {"impl_283"}
// Dependencies: {}
impl Decode for Form { const MIME : (mime :: Name < 'static > , mime :: Name < 'static >) = (mime :: APPLICATION , mime :: WWW_FORM_URLENCODED) ; const WITH_NO_CONTENT_TYPE : bool = true ; fn decode < B : Buf , T : DeserializeOwned > (buf : B) -> Result < T , BoxError > { serde_urlencoded :: from_reader (buf . reader ()) . map_err (Into :: into) } }
};
}
