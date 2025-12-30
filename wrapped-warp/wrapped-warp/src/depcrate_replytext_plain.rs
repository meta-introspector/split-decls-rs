// Generated macro for text_plain (function)
macro_rules! Depcrate_replytext_plain {
() => {
// Module: crate::reply
// Provides: {"text_plain"}
// Dependencies: {}
fn text_plain < T : Into < Body > > (body : T) -> Response { let mut response = :: http :: Response :: new (body . into ()) ; response . headers_mut () . insert (CONTENT_TYPE , HeaderValue :: from_static ("text/plain; charset=utf-8") ,) ; response }
};
}
