// Generated macro for Policy (trait)
macro_rules! Depcrate_hedgePolicy {
() => {
// Module: crate::hedge
// Provides: {"Policy"}
// Dependencies: {}
# [doc = " A policy which describes which requests can be cloned and then whether those"] # [doc = " requests should be retried."] pub trait Policy < Request > { # [doc = " Called when the request is first received to determine if the request is retryable."] fn clone_request (& self , req : & Request) -> Option < Request > ; # [doc = " Called after the hedge timeout to determine if the hedge retry should be issued."] fn can_retry (& self , req : & Request) -> bool ; }
};
}
