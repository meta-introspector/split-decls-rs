// Generated macro for impl_1078 (impl)
macro_rules! Depcrate_services_redirectimpl_1078 {
() => {
// Module: crate::services::redirect
// Provides: {"impl_1078"}
// Dependencies: {}
impl < ResBody > Redirect < ResBody > { # [doc = " Create a new [`Redirect`] that uses a [`307 Temporary Redirect`][mdn] status code."] # [doc = ""] # [doc = " [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/307"] pub fn temporary (uri : Uri) -> Self { Self :: with_status_code (StatusCode :: TEMPORARY_REDIRECT , uri) } # [doc = " Create a new [`Redirect`] that uses a [`308 Permanent Redirect`][mdn] status code."] # [doc = ""] # [doc = " [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/308"] pub fn permanent (uri : Uri) -> Self { Self :: with_status_code (StatusCode :: PERMANENT_REDIRECT , uri) } # [doc = " Create a new [`Redirect`] that uses the given status code."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " - If `status_code` isn't a [redirection status code][mdn] (3xx)."] # [doc = " - If `uri` isn't a valid [`HeaderValue`]."] # [doc = ""] # [doc = " [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#redirection_messages"] pub fn with_status_code (status_code : StatusCode , uri : Uri) -> Self { assert ! (status_code . is_redirection () , "not a redirection status code") ; Self { status_code , location : HeaderValue :: try_from (uri . to_string ()) . expect ("URI isn't a valid header value") , _marker : PhantomData , } } }
};
}
