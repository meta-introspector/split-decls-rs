// Generated macro for MakeRequestId (trait)
macro_rules! Depcrate_request_idMakeRequestId {
() => {
// Module: crate::request_id
// Provides: {"MakeRequestId"}
// Dependencies: {}
# [doc = " Trait for producing [`RequestId`]s."] # [doc = ""] # [doc = " Used by [`SetRequestId`]."] pub trait MakeRequestId { # [doc = " Try and produce a [`RequestId`] from the request."] fn make_request_id < B > (& mut self , request : & Request < B >) -> Option < RequestId > ; }
};
}
