// Generated macro for impl_110 (impl)
macro_rules! Depcrate_valuableimpl_110 {
() => {
// Module: crate::valuable
// Provides: {"impl_110"}
// Dependencies: {}
impl < T : Valuable > Valuable for Wrapping < T > { fn as_value (& self) -> Value < '_ > { self . 0 . as_value () } fn visit (& self , visit : & mut dyn Visit) { self . 0 . visit (visit) ; } }
};
}
