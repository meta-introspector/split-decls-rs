// Generated macro for round (function)
macro_rules! Depcrate_parsing_iso8601round {
() => {
// Module: crate::parsing::iso8601
// Provides: {"round"}
// Dependencies: {}
# [doc = " Round wrapper that uses hardware implementation if `std` is available, falling back to manual"] # [doc = " implementation for `no_std`"] # [inline] fn round (value : f64) -> f64 { # [cfg (feature = "std")] { value . round () } # [cfg (not (feature = "std"))] { debug_assert ! (value . is_sign_positive () && ! value . is_nan ()) ; let f = value % 1. ; if f < 0.5 { value - f } else { value - f + 1. } } }
};
}
