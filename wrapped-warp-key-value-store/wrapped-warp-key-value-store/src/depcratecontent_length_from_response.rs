// Generated macro for content_length_from_response (function)
macro_rules! Depcratecontent_length_from_response {
() => {
// Module: crate
// Provides: {"content_length_from_response"}
// Dependencies: {}
fn content_length_from_response < B > (response : & Response < B >) -> Option < HeaderValue > where B : HttpBody , { response . body () . size_hint () . exact () . map (| size | HeaderValue :: from_str (& size . to_string ()) . unwrap ()) }
};
}
