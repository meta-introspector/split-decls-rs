// Generated macro for tests (module)
macro_rules! Depcrate_wrap_algorithmstests {
() => {
// Module: crate::wrap_algorithms
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [derive (Debug , PartialEq)] struct Word (f64) ; # [rustfmt :: skip] impl Fragment for Word { fn width (& self) -> f64 { self . 0 } fn whitespace_width (& self) -> f64 { 1.0 } fn penalty_width (& self) -> f64 { 0.0 } } # [test] fn wrap_string_longer_than_f64 () { let words = vec ! [Word (1e307) , Word (2e307) , Word (3e307) , Word (4e307) , Word (5e307) , Word (6e307) ,] ; assert_eq ! (wrap_first_fit (& words , & [15e307]) , & [vec ! [Word (1e307) , Word (2e307) , Word (3e307) , Word (4e307) , Word (5e307)] , vec ! [Word (6e307)]]) ; } }
};
}
