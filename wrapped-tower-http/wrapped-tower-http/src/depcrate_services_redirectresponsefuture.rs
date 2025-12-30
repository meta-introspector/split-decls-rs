// Generated macro for ResponseFuture (struct)
macro_rules! Depcrate_services_redirectResponseFuture {
() => {
// Module: crate::services::redirect
// Provides: {"ResponseFuture"}
// Dependencies: {}
# [doc = " Response future of [`Redirect`]."] # [derive (Debug)] pub struct ResponseFuture < ResBody > { location : Option < HeaderValue > , status_code : StatusCode , _marker : PhantomData < fn () -> ResBody > , }
};
}
