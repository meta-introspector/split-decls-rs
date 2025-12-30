// Generated macro for Redirect (struct)
macro_rules! Depcrate_services_redirectRedirect {
() => {
// Module: crate::services::redirect
// Provides: {"Redirect"}
// Dependencies: {}
# [doc = " Service that redirects all requests."] # [doc = ""] # [doc = " See the [module docs](crate::services::redirect) for more details."] pub struct Redirect < ResBody > { status_code : StatusCode , location : HeaderValue , _marker : PhantomData < fn () -> ResBody > , }
};
}
