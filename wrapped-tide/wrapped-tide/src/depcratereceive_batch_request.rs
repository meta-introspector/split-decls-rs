// Generated macro for receive_batch_request (function)
macro_rules! Depcratereceive_batch_request {
() => {
// Module: crate
// Provides: {"receive_batch_request"}
// Dependencies: {}
# [doc = " Convert a Tide request to a GraphQL batch request."] pub async fn receive_batch_request < State : Clone + Send + Sync + 'static > (request : Request < State > ,) -> tide :: Result < async_graphql :: BatchRequest > { receive_batch_request_opts (request , Default :: default ()) . await }
};
}
