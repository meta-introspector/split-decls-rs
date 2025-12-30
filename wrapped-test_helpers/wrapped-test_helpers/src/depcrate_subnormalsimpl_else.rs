// Generated macro for impl_else (macro)
macro_rules! Depcrate_subnormalsimpl_else {
() => {
// Module: crate::subnormals
// Provides: {"impl_else"}
// Dependencies: {}
macro_rules ! impl_else { { $ ($ ty : ty) ,* } => { $ (impl FlushSubnormals for $ ty { }) * } }
};
}
