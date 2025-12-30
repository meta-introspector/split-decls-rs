// Generated macro for AsService (struct)
macro_rules! Depcrate_make_make_serviceAsService {
() => {
// Module: crate::make::make_service
// Provides: {"AsService"}
// Dependencies: {}
# [doc = " Service returned by [`MakeService::as_service`][as]."] # [doc = ""] # [doc = " See the documentation on [`as_service`][as] for details."] # [doc = ""] # [doc = " [as]: MakeService::as_service"] pub struct AsService < 'a , M , Request > { make : & 'a mut M , _marker : PhantomData < Request > , }
};
}
