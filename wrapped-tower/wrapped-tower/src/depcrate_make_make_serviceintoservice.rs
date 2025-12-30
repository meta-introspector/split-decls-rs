// Generated macro for IntoService (struct)
macro_rules! Depcrate_make_make_serviceIntoService {
() => {
// Module: crate::make::make_service
// Provides: {"IntoService"}
// Dependencies: {}
# [doc = " Service returned by [`MakeService::into_service`][into]."] # [doc = ""] # [doc = " See the documentation on [`into_service`][into] for details."] # [doc = ""] # [doc = " [into]: MakeService::into_service"] pub struct IntoService < M , Request > { make : M , _marker : PhantomData < Request > , }
};
}
