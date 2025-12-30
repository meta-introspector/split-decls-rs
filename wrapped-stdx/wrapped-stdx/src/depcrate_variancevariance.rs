// Generated macro for variance (function)
macro_rules! Depcrate_variancevariance {
() => {
// Module: crate::variance
// Provides: {"variance"}
// Dependencies: {}
# [doc = " Construct a variance marker; equivalent to [`Default::default`]."] # [doc = ""] # [doc = " This type can be any of the following. You generally should not need to explicitly name the"] # [doc = " type, however."] # [doc = ""] # [doc = " - [`PhantomCovariant`]"] # [doc = " - [`PhantomContravariant`]"] # [doc = " - [`PhantomInvariant`]"] # [doc = " - [`PhantomCovariantLifetime`]"] # [doc = " - [`PhantomContravariantLifetime`]"] # [doc = " - [`PhantomInvariantLifetime`]"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![feature(phantom_variance_markers)]"] # [doc = ""] # [doc = " use core::marker::{PhantomCovariant, variance};"] # [doc = ""] # [doc = " struct BoundFn<F, P, R>"] # [doc = " where"] # [doc = "     F: Fn(P) -> R,"] # [doc = " {"] # [doc = "     function: F,"] # [doc = "     parameter: P,"] # [doc = "     return_value: PhantomCovariant<R>,"] # [doc = " }"] # [doc = ""] # [doc = " let bound_fn = BoundFn {"] # [doc = "     function: core::convert::identity,"] # [doc = "     parameter: 5u8,"] # [doc = "     return_value: variance(),"] # [doc = " };"] # [doc = " ```"] pub const fn variance < T > () -> T where T : Variance , { T :: VALUE }
};
}
