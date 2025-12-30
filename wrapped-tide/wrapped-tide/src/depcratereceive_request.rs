// Generated macro for receive_request (function)
macro_rules! Depcratereceive_request {
() => {
// Module: crate
// Provides: {"receive_request"}
// Dependencies: {}
# [doc = " Convert a Tide request to a GraphQL request."] pub async fn receive_request < State : Clone + Send + Sync + 'static > (request : Request < State > ,) -> tide :: Result < async_graphql :: Request > { receive_request_opts (request , Default :: default ()) . await }
};
}
