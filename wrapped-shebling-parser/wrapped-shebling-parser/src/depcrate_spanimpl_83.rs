// Generated macro for impl_83 (impl)
macro_rules! Depcrate_spanimpl_83 {
() => {
// Module: crate::span
// Provides: {"impl_83"}
// Dependencies: {}
impl ParseDiags { pub (crate) fn new () -> Self { Self (RefCell :: new (vec ! [])) } fn push (& self , builder : DiagnosticBuilder) { self . 0 . borrow_mut () . push (builder . build ()) ; } }
};
}
