// Generated macro for ResponseForPanic (trait)
macro_rules! Depcrate_catch_panicResponseForPanic {
() => {
// Module: crate::catch_panic
// Provides: {"ResponseForPanic"}
// Dependencies: {}
# [doc = " Trait for creating responses from panics."] pub trait ResponseForPanic : Clone { # [doc = " The body type used for responses to panics."] type ResponseBody ; # [doc = " Create a response from the panic error."] fn response_for_panic (& mut self , err : Box < dyn Any + Send + 'static > ,) -> Response < Self :: ResponseBody > ; }
};
}
