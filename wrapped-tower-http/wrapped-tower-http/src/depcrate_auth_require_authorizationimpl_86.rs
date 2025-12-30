// Generated macro for impl_86 (impl)
macro_rules! Depcrate_auth_require_authorizationimpl_86 {
() => {
// Module: crate::auth::require_authorization
// Provides: {"impl_86"}
// Dependencies: {}
impl < B , ResBody > ValidateRequest < B > for Basic < ResBody > where ResBody : Default , { type ResponseBody = ResBody ; fn validate (& mut self , request : & mut Request < B >) -> Result < () , Response < Self :: ResponseBody > > { match request . headers () . get (header :: AUTHORIZATION) { Some (actual) if actual == self . header_value => Ok (()) , _ => { let mut res = Response :: new (ResBody :: default ()) ; * res . status_mut () = StatusCode :: UNAUTHORIZED ; res . headers_mut () . insert (header :: WWW_AUTHENTICATE , "Basic" . parse () . unwrap ()) ; Err (res) } } } }
};
}
