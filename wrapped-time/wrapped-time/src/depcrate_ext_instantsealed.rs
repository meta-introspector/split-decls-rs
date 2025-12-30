// Generated macro for sealed (module)
macro_rules! Depcrate_ext_instantsealed {
() => {
// Module: crate::ext::instant
// Provides: {"sealed"}
// Dependencies: {}
# [doc = " Sealed trait to prevent downstream implementations."] mod sealed { # [doc = " A trait that cannot be implemented by downstream users."] pub trait Sealed : Sized { } impl Sealed for std :: time :: Instant { } }
};
}
