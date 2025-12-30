// Generated macro for GraphQLEndpoint (struct)
macro_rules! DepcrateGraphQLEndpoint {
() => {
// Module: crate
// Provides: {"GraphQLEndpoint"}
// Dependencies: {}
# [doc = " A GraphQL endpoint."] # [doc = ""] # [doc = " This is created with the [`endpoint`](fn.endpoint.html) function."] # [non_exhaustive] pub struct GraphQLEndpoint < E > { # [doc = " The graphql executor"] pub executor : E , # [doc = " The multipart options of the endpoint."] pub opts : MultipartOptions , # [doc = " Whether to support batch requests in the endpoint."] pub batch : bool , }
};
}
