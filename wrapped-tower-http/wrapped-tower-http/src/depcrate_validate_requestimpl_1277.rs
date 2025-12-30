// Generated macro for impl_1277 (impl)
macro_rules! Depcrate_validate_requestimpl_1277 {
() => {
// Module: crate::validate_request
// Provides: {"impl_1277"}
// Dependencies: {}
impl < B , F , ResBody > ValidateRequest < B > for F where F : FnMut (& mut Request < B >) -> Result < () , Response < ResBody > > , { type ResponseBody = ResBody ; fn validate (& mut self , request : & mut Request < B >) -> Result < () , Response < Self :: ResponseBody > > { self (request) } }
};
}
