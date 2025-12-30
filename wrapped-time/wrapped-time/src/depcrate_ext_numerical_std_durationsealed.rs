// Generated macro for sealed (module)
macro_rules! Depcrate_ext_numerical_std_durationsealed {
() => {
// Module: crate::ext::numerical_std_duration
// Provides: {"sealed"}
// Dependencies: {}
# [doc = " Sealed trait to prevent downstream implementations."] mod sealed { # [doc = " A trait that cannot be implemented by downstream users."] pub trait Sealed { } impl Sealed for u64 { } impl Sealed for f64 { } }
};
}
