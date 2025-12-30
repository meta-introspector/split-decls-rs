// Generated macro for Policy (trait)
macro_rules! Depcrate_hedge_selectPolicy {
() => {
// Module: crate::hedge::select
// Provides: {"Policy"}
// Dependencies: {}
# [doc = " A policy which decides which requests can be cloned and sent to the B"] # [doc = " service."] pub trait Policy < Request > { fn clone_request (& self , req : & Request) -> Option < Request > ; }
};
}
