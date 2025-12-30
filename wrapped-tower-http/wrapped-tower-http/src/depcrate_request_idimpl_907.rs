// Generated macro for impl_907 (impl)
macro_rules! Depcrate_request_idimpl_907 {
() => {
// Module: crate::request_id
// Provides: {"impl_907"}
// Dependencies: {}
impl MakeRequestId for MakeRequestUuid { fn make_request_id < B > (& mut self , _request : & Request < B >) -> Option < RequestId > { let request_id = Uuid :: new_v4 () . to_string () . parse () . unwrap () ; Some (RequestId :: new (request_id)) } }
};
}
