// Generated macro for impl_81 (impl)
macro_rules! Depcrate_auth_require_authorizationimpl_81 {
() => {
// Module: crate::auth::require_authorization
// Provides: {"impl_81"}
// Dependencies: {}
impl < B , ResBody > ValidateRequest < B > for Bearer < ResBody > where ResBody : Default , { type ResponseBody = ResBody ; fn validate (& mut self , request : & mut Request < B >) -> Result < () , Response < Self :: ResponseBody > > { match request . headers () . get (header :: AUTHORIZATION) { Some (actual) if actual == self . header_value => Ok (()) , _ => { let mut res = Response :: new (ResBody :: default ()) ; * res . status_mut () = StatusCode :: UNAUTHORIZED ; Err (res) } } } }
};
}
