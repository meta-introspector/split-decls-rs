// Generated macro for MakeHeaderValue (trait)
macro_rules! Depcrate_set_headerMakeHeaderValue {
() => {
// Module: crate::set_header
// Provides: {"MakeHeaderValue"}
// Dependencies: {}
# [doc = " Trait for producing header values."] # [doc = ""] # [doc = " Used by [`SetRequestHeader`] and [`SetResponseHeader`]."] # [doc = ""] # [doc = " This trait is implemented for closures with the correct type signature. Typically users will"] # [doc = " not have to implement this trait for their own types."] # [doc = ""] # [doc = " It is also implemented directly for [`HeaderValue`]. When a fixed header value should be added"] # [doc = " to all responses, it can be supplied directly to the middleware."] pub trait MakeHeaderValue < T > { # [doc = " Try to create a header value from the request or response."] fn make_header_value (& mut self , message : & T) -> Option < HeaderValue > ; }
};
}
