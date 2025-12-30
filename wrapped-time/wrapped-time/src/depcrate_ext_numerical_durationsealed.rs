// Generated macro for sealed (module)
macro_rules! Depcrate_ext_numerical_durationsealed {
() => {
// Module: crate::ext::numerical_duration
// Provides: {"sealed"}
// Dependencies: {}
# [doc = " Sealed trait to prevent downstream implementations."] mod sealed { # [doc = " A trait that cannot be implemented by downstream users."] pub trait Sealed { } impl Sealed for i64 { } impl Sealed for f64 { } }
};
}
