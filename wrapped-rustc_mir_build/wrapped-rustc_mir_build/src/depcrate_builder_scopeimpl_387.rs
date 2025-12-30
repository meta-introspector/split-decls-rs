// Generated macro for impl_387 (impl)
macro_rules! Depcrate_builder_scopeimpl_387 {
() => {
// Module: crate::builder::scope
// Provides: {"impl_387"}
// Dependencies: {}
impl < 'tcx > DropTreeBuilder < 'tcx > for ExitScopes { fn make_block (cfg : & mut CFG < 'tcx >) -> BasicBlock { cfg . start_new_block () } fn link_entry_point (cfg : & mut CFG < 'tcx > , from : BasicBlock , to : BasicBlock) { let term = cfg . block_data_mut (from) . terminator_mut () ; if let TerminatorKind :: UnwindResume = term . kind { term . kind = TerminatorKind :: Goto { target : to } ; } else { span_bug ! (term . source_info . span , "unexpected dummy terminator kind: {:?}" , term . kind) ; } } }
};
}
