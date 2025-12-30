// Generated macro for impl_92 (impl)
macro_rules! Depcrate_errorsimpl_92 {
() => {
// Module: crate::errors
// Provides: {"impl_92"}
// Dependencies: {}
impl < G : EmissionGuarantee > Diagnostic < '_ , G > for TestOutput { fn into_diag (self , dcx : DiagCtxtHandle < '_ > , level : Level) -> Diag < '_ , G > { let TestOutput { span , kind , content } = self ; # [allow (rustc :: untranslatable_diagnostic)] Diag :: new (dcx , level , format ! ("{kind}({content})")) . with_span (span) } }
};
}
