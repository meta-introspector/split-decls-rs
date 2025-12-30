// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
# [async_trait] impl < E , TideState > tide :: Endpoint < TideState > for GraphQLEndpoint < E > where E : Executor , TideState : Clone + Send + Sync + 'static , { async fn call (& self , request : Request < TideState >) -> tide :: Result { respond (self . executor . execute_batch (if self . batch { receive_batch_request_opts (request , self . opts) . await } else { receive_request_opts (request , self . opts) . await . map (Into :: into) } ?) . await ,) } }
};
}
