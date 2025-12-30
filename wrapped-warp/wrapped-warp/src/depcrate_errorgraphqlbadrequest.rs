// Generated macro for GraphQLBadRequest (struct)
macro_rules! Depcrate_errorGraphQLBadRequest {
() => {
// Module: crate::error
// Provides: {"GraphQLBadRequest"}
// Dependencies: {}
# [doc = " Bad request error."] # [doc = ""] # [doc = " It's a wrapper of `async_graphql::ParseRequestError`. It is also a `Reply` -"] # [doc = " by default it just returns a response containing the error message in plain"] # [doc = " text."] # [derive (Debug)] pub struct GraphQLBadRequest (pub ParseRequestError) ;
};
}
