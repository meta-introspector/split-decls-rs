// Generated macro for impl_509 (impl)
macro_rules! Depcrate_instantimpl_509 {
() => {
// Module: crate::instant
// Provides: {"impl_509"}
// Dependencies: {}
impl Sub < Duration > for Instant { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This function may panic if the resulting point in time cannot be represented by the"] # [doc = " underlying data structure."] # [inline] fn sub (self , duration : Duration) -> Self :: Output { if duration . is_positive () { # [expect (clippy :: unchecked_duration_subtraction)] Self (self . 0 - duration . unsigned_abs ()) } else if duration . is_negative () { Self (self . 0 + duration . unsigned_abs ()) } else { debug_assert ! (duration . is_zero ()) ; self } } }
};
}
