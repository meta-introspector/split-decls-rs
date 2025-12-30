// Generated macro for local_decls_for_sig (function)
macro_rules! Depcrate_shimlocal_decls_for_sig {
() => {
// Module: crate::shim
// Provides: {"local_decls_for_sig"}
// Dependencies: {}
fn local_decls_for_sig < 'tcx > (sig : & ty :: FnSig < 'tcx > , span : Span ,) -> IndexVec < Local , LocalDecl < 'tcx > > { iter :: once (LocalDecl :: new (sig . output () , span)) . chain (sig . inputs () . iter () . map (| ity | LocalDecl :: new (* ity , span) . immutable ())) . collect () }
};
}
