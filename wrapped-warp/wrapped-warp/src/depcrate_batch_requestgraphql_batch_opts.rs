// Generated macro for graphql_batch_opts (function)
macro_rules! Depcrate_batch_requestgraphql_batch_opts {
() => {
// Module: crate::batch_request
// Provides: {"graphql_batch_opts"}
// Dependencies: {}
# [doc = " Similar to graphql_batch, but you can set the options with"] # [doc = " :`async_graphql::MultipartOptions`."] pub fn graphql_batch_opts < E > (executor : E , opts : MultipartOptions ,) -> impl Filter < Extract = ((E , BatchRequest) ,) , Error = Rejection > + Clone where E : Executor , { warp :: any () . and (warp :: get () . and (warp :: filters :: query :: raw ()) . and_then (| query_string : String | async move { async_graphql :: http :: parse_query_string (& query_string) . map (Into :: into) . map_err (| e | warp :: reject :: custom (GraphQLBadRequest (e))) } ,)) . or (warp :: post () . and (warp :: header :: optional :: < String > ("content-type")) . and (warp :: body :: stream ()) . and_then (move | content_type , body | async move { async_graphql :: http :: receive_batch_body (content_type , TryStreamExt :: map_err (body , io :: Error :: other) . map_ok (| mut buf | { let remaining = Buf :: remaining (& buf) ; Buf :: copy_to_bytes (& mut buf , remaining) }) . into_async_read () , opts ,) . await . map_err (| e | warp :: reject :: custom (GraphQLBadRequest (e))) })) . unify () . map (move | res | (executor . clone () , res)) }
};
}
