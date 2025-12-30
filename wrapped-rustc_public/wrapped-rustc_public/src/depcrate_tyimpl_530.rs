// Generated macro for impl_530 (impl)
macro_rules! Depcrate_tyimpl_530 {
() => {
// Module: crate::ty
// Provides: {"impl_530"}
// Dependencies: {}
impl FnSig { pub fn output (& self) -> Ty { self . inputs_and_output [self . inputs_and_output . len () - 1] } pub fn inputs (& self) -> & [Ty] { & self . inputs_and_output [.. self . inputs_and_output . len () - 1] } }
};
}
