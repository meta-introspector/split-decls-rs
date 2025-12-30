// Generated macro for RequestBodyLimit (struct)
macro_rules! Depcrate_limit_serviceRequestBodyLimit {
() => {
// Module: crate::limit::service
// Provides: {"RequestBodyLimit"}
// Dependencies: {}
# [doc = " Middleware that intercepts requests with body lengths greater than the"] # [doc = " configured limit and converts them into `413 Payload Too Large` responses."] # [doc = ""] # [doc = " See the [module docs](crate::limit) for an example."] # [derive (Clone , Copy , Debug)] pub struct RequestBodyLimit < S > { pub (crate) inner : S , pub (crate) limit : usize , }
};
}
