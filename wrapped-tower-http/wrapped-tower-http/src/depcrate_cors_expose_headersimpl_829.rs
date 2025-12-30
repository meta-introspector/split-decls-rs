// Generated macro for impl_829 (impl)
macro_rules! Depcrate_cors_expose_headersimpl_829 {
() => {
// Module: crate::cors::expose_headers
// Provides: {"impl_829"}
// Dependencies: {}
impl ExposeHeaders { # [doc = " Expose any / all headers by sending a wildcard (`*`)"] # [doc = ""] # [doc = " See [`CorsLayer::expose_headers`] for more details."] # [doc = ""] # [doc = " [`CorsLayer::expose_headers`]: super::CorsLayer::expose_headers"] pub fn any () -> Self { Self (ExposeHeadersInner :: Const (Some (WILDCARD))) } # [doc = " Set multiple exposed header names"] # [doc = ""] # [doc = " See [`CorsLayer::expose_headers`] for more details."] # [doc = ""] # [doc = " [`CorsLayer::expose_headers`]: super::CorsLayer::expose_headers"] pub fn list < I > (headers : I) -> Self where I : IntoIterator < Item = HeaderName > , { Self (ExposeHeadersInner :: Const (separated_by_commas (headers . into_iter () . map (Into :: into) ,))) } # [allow (clippy :: borrow_interior_mutable_const)] pub (super) fn is_wildcard (& self) -> bool { matches ! (& self . 0 , ExposeHeadersInner :: Const (Some (v)) if v == WILDCARD) } pub (super) fn to_header (& self , _parts : & RequestParts) -> Option < (HeaderName , HeaderValue) > { let expose_headers = match & self . 0 { ExposeHeadersInner :: Const (v) => v . clone () ? , } ; Some ((header :: ACCESS_CONTROL_EXPOSE_HEADERS , expose_headers)) } }
};
}
