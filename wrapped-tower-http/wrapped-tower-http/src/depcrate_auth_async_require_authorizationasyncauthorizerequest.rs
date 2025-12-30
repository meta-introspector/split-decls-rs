// Generated macro for AsyncAuthorizeRequest (trait)
macro_rules! Depcrate_auth_async_require_authorizationAsyncAuthorizeRequest {
() => {
// Module: crate::auth::async_require_authorization
// Provides: {"AsyncAuthorizeRequest"}
// Dependencies: {}
# [doc = " Trait for authorizing requests."] pub trait AsyncAuthorizeRequest < B > { # [doc = " The type of request body returned by `authorize`."] # [doc = ""] # [doc = " Set this to `B` unless you need to change the request body type."] type RequestBody ; # [doc = " The body type used for responses to unauthorized requests."] type ResponseBody ; # [doc = " The Future type returned by `authorize`"] type Future : Future < Output = Result < Request < Self :: RequestBody > , Response < Self :: ResponseBody > > > ; # [doc = " Authorize the request."] # [doc = ""] # [doc = " If the future resolves to `Ok(request)` then the request is allowed through, otherwise not."] fn authorize (& mut self , request : Request < B >) -> Self :: Future ; }
};
}
