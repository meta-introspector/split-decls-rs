// Generated macro for ValidateRequest (trait)
macro_rules! Depcrate_validate_requestValidateRequest {
() => {
// Module: crate::validate_request
// Provides: {"ValidateRequest"}
// Dependencies: {}
# [doc = " Trait for validating requests."] pub trait ValidateRequest < B > { # [doc = " The body type used for responses to unvalidated requests."] type ResponseBody ; # [doc = " Validate the request."] # [doc = ""] # [doc = " If `Ok(())` is returned then the request is allowed through, otherwise not."] fn validate (& mut self , request : & mut Request < B >) -> Result < () , Response < Self :: ResponseBody > > ; }
};
}
