// Generated macro for sealed (module)
macro_rules! Depcrate_replysealed {
() => {
// Module: crate::reply
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { use super :: { Reply , Response } ; # [allow (missing_debug_implementations)] pub struct Reply_ (pub (crate) Response) ; impl Reply for Reply_ { # [inline] fn into_response (self) -> Response { self . 0 } } # [allow (missing_debug_implementations)] pub struct Internal ; pub trait BoxedReply { fn boxed_into_response (self : Box < Self > , internal : Internal) -> Response ; } impl < T : Reply > BoxedReply for T { fn boxed_into_response (self : Box < Self > , _ : Internal) -> Response { (* self) . into_response () } } }
};
}
