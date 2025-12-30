// Generated macro for graphql_batch (function)
macro_rules! Depcrate_batch_requestgraphql_batch {
() => {
// Module: crate::batch_request
// Provides: {"graphql_batch"}
// Dependencies: {}
# [doc = " GraphQL batch request filter"] # [doc = ""] # [doc = " It outputs a tuple containing the `async_graphql::Executor` and"] # [doc = " `async_graphql::BatchRequest`."] pub fn graphql_batch < E > (executor : E ,) -> impl Filter < Extract = ((E , BatchRequest) ,) , Error = Rejection > + Clone where E : Executor , { graphql_batch_opts (executor , Default :: default ()) }
};
}
