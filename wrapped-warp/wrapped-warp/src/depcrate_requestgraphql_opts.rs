// Generated macro for graphql_opts (function)
macro_rules! Depcrate_requestgraphql_opts {
() => {
// Module: crate::request
// Provides: {"graphql_opts"}
// Dependencies: {}
# [doc = " Similar to graphql, but you can set the options"] # [doc = " `async_graphql::MultipartOptions`."] pub fn graphql_opts < E > (executor : E , opts : MultipartOptions ,) -> impl Filter < Extract = ((E , Request) ,) , Error = Rejection > + Clone where E : Executor , { graphql_batch_opts (executor , opts) . and_then (| (schema , batch) : (_ , BatchRequest) | async move { < Result < _ , Rejection > > :: Ok ((schema , batch . into_single () . map_err (| e | warp :: reject :: custom (GraphQLBadRequest (e))) ? ,)) }) }
};
}
