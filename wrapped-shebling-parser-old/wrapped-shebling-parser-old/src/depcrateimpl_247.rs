// Generated macro for impl_247 (impl)
macro_rules! Depcrateimpl_247 {
() => {
// Module: crate
// Provides: {"impl_247"}
// Dependencies: {}
impl ParseContext { fn new () -> Self { Self { diags : RefCell :: new (vec ! []) , } } fn diag (& self , builder : ParseDiagnosticBuilder) { self . diags . borrow_mut () . push (builder . build ()) ; } fn take_diags (& self) -> Vec < ParseDiagnostic > { self . diags . take () } fn extend_diags (& self , other : Self) { self . diags . borrow_mut () . extend (other . take_diags ()) ; } }
};
}
