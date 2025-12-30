// Generated macro for impl_850 (impl)
macro_rules! Depcrate_cors_varyimpl_850 {
() => {
// Module: crate::cors::vary
// Provides: {"impl_850"}
// Dependencies: {}
impl Vary { # [doc = " Set the list of header names to return as vary header values"] # [doc = ""] # [doc = " See [`CorsLayer::vary`] for more details."] # [doc = ""] # [doc = " [`CorsLayer::vary`]: super::CorsLayer::vary"] pub fn list < I > (headers : I) -> Self where I : IntoIterator < Item = HeaderName > , { Self (headers . into_iter () . map (Into :: into) . collect ()) } pub (super) fn to_header (& self) -> Option < (HeaderName , HeaderValue) > { let values = & self . 0 ; let mut res = values . first () ? . as_bytes () . to_owned () ; for val in & values [1 ..] { res . extend_from_slice (b", ") ; res . extend_from_slice (val . as_bytes ()) ; } let header_val = HeaderValue :: from_bytes (& res) . expect ("comma-separated list of HeaderValues is always a valid HeaderValue") ; Some ((header :: VARY , header_val)) } }
};
}
