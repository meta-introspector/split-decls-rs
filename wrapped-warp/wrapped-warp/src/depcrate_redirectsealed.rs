// Generated macro for sealed (module)
macro_rules! Depcrate_redirectsealed {
() => {
// Module: crate::redirect
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { use bytes :: Bytes ; use http :: { header :: HeaderValue , Uri } ; # [doc = " Trait for redirect locations. Currently only a `Uri` can be used in"] # [doc = " redirect."] # [doc = " This sealed trait exists to allow adding possibly new impls so other"] # [doc = " arguments could be accepted, like maybe just `warp::redirect(\"/v2\")`."] pub trait AsLocation : Sealed { } pub trait Sealed { fn header_value (self) -> HeaderValue ; } impl AsLocation for Uri { } impl Sealed for Uri { fn header_value (self) -> HeaderValue { let bytes = Bytes :: from (self . to_string ()) ; HeaderValue :: from_maybe_shared (bytes) . expect ("Uri is a valid HeaderValue") } } }
};
}
