// Generated macro for impl_1143 (impl)
macro_rules! Depcrate_parse_sessionimpl_1143 {
() => {
// Module: crate::parse::session
// Provides: {"impl_1143"}
// Dependencies: {}
impl ParseSess { pub (super) fn emit_diagnostics (& self , diagnostics : Vec < Diag < '_ > >) { for diagnostic in diagnostics { diagnostic . emit () ; } } pub (super) fn can_reset_errors (& self) -> bool { self . can_reset_errors . load (Ordering :: Acquire) } pub (super) fn has_errors (& self) -> bool { self . raw_psess . dcx () . has_errors () . is_some () } pub (super) fn reset_errors (& self) { self . raw_psess . dcx () . reset_err_count () ; } }
};
}
