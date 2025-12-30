// Generated macro for impl_is_negative (macro)
macro_rules! Depcrate_vsockimpl_is_negative {
() => {
// Module: crate::vsock
// Provides: {"impl_is_negative"}
// Dependencies: {}
macro_rules ! impl_is_negative { ($ ($ t : ident) *) => ($ (impl IsNegative for $ t { fn is_negative (& self) -> bool { * self < 0 } fn negate (& self) -> i32 { i32 :: try_from (- (* self)) . unwrap () } }) *) }
};
}
