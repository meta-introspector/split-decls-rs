// Generated macro for impl_debug_strict_add (macro)
macro_rules! Depcrate_int_overflowimpl_debug_strict_add {
() => {
// Module: crate::int_overflow
// Provides: {"impl_debug_strict_add"}
// Dependencies: {}
macro_rules ! impl_debug_strict_add { ($ ($ ty : ty) *) => { $ (impl DebugStrictAdd for $ ty { # [inline] fn debug_strict_add (self , other : Self) -> Self { if cfg ! (debug_assertions) { self + other } else { self . wrapping_add (other) } } }) * } ; }
};
}
