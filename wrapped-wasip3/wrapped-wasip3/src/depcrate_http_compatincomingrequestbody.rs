// Generated macro for IncomingRequestBody (type)
macro_rules! Depcrate_http_compatIncomingRequestBody {
() => {
// Module: crate::http_compat
// Provides: {"IncomingRequestBody"}
// Dependencies: {}
# [doc = " The body type used for incoming HTTP requests."] # [doc = ""] # [doc = " This is a type alias for [`IncomingBody`] specialized with"] # [doc = " [`types::Request`], representing the structured payload of an"] # [doc = " inbound request as it is received and processed."] # [doc = ""] # [doc = " This type is typically used by components that consume HTTP"] # [doc = " requests and need to read or stream the request body."] pub type IncomingRequestBody = IncomingBody < types :: Request > ;
};
}
