// Generated macro for impl_545 (impl)
macro_rules! Depcrate_tyimpl_545 {
() => {
// Module: crate::ty
// Provides: {"impl_545"}
// Dependencies: {}
impl FnSig { pub fn output (& self) -> Ty { self . inputs_and_output [self . inputs_and_output . len () - 1] } pub fn inputs (& self) -> & [Ty] { & self . inputs_and_output [.. self . inputs_and_output . len () - 1] } }
};
}
