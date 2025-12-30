// Generated macro for graphql (function)
macro_rules! Depcrategraphql {
() => {
// Module: crate
// Provides: {"graphql"}
// Dependencies: {}
# [doc = " Create a new GraphQL endpoint with the executor."] # [doc = ""] # [doc = " Default multipart options are used and batch operations are supported."] pub fn graphql < E > (executor : E) -> GraphQLEndpoint < E > { GraphQLEndpoint { executor , opts : MultipartOptions :: default () , batch : true , } }
};
}
