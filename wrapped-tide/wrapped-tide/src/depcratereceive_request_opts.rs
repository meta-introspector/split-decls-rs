// Generated macro for receive_request_opts (function)
macro_rules! Depcratereceive_request_opts {
() => {
// Module: crate
// Provides: {"receive_request_opts"}
// Dependencies: {}
# [doc = " Convert a Tide request to a GraphQL request with options on how to receive"] # [doc = " multipart."] pub async fn receive_request_opts < State : Clone + Send + Sync + 'static > (request : Request < State > , opts : MultipartOptions ,) -> tide :: Result < async_graphql :: Request > { receive_batch_request_opts (request , opts) . await ? . into_single () . map_err (| e | tide :: Error :: new (StatusCode :: BadRequest , e)) }
};
}
