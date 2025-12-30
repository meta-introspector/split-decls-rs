// Generated macro for receive_batch_request_opts (function)
macro_rules! Depcratereceive_batch_request_opts {
() => {
// Module: crate
// Provides: {"receive_batch_request_opts"}
// Dependencies: {}
# [doc = " Convert a Tide request to a GraphQL batch request with options on how to"] # [doc = " receive multipart."] pub async fn receive_batch_request_opts < State : Clone + Send + Sync + 'static > (mut request : Request < State > , opts : MultipartOptions ,) -> tide :: Result < async_graphql :: BatchRequest > { if request . method () == Method :: Get { async_graphql :: http :: parse_query_string (request . url () . query () . unwrap_or_default ()) . map (Into :: into) . map_err (| err | tide :: Error :: new (StatusCode :: BadRequest , err)) } else if request . method () == Method :: Post { let body = request . take_body () ; let content_type = request . header (headers :: CONTENT_TYPE) . and_then (| values | values . get (0)) . map (HeaderValue :: as_str) ; async_graphql :: http :: receive_batch_body (content_type , body , opts) . await . map_err (| e | { tide :: Error :: new (match & e { ParseRequestError :: PayloadTooLarge => StatusCode :: PayloadTooLarge , _ => StatusCode :: BadRequest , } , e ,) }) } else { Err (tide :: Error :: from_str (StatusCode :: MethodNotAllowed , "GraphQL only supports GET and POST requests" ,)) } }
};
}
