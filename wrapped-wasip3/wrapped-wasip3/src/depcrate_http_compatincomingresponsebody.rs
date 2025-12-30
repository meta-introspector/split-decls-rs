// Generated macro for IncomingResponseBody (type)
macro_rules! Depcrate_http_compatIncomingResponseBody {
() => {
// Module: crate::http_compat
// Provides: {"IncomingResponseBody"}
// Dependencies: {}
# [doc = " The body type used for incoming HTTP responses."] # [doc = ""] # [doc = " This is a type alias for [`IncomingBody`] specialized with"] # [doc = " [`types::Response`], representing the structured payload of an"] # [doc = " inbound response as it is received and processed."] # [doc = ""] # [doc = " This type is typically used by components that handle HTTP"] # [doc = " responses and need to read or stream the response body."] pub type IncomingResponseBody = IncomingBody < types :: Response > ;
};
}
