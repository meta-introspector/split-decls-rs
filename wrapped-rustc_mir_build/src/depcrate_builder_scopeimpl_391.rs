// Generated macro for impl_391 (impl)
macro_rules! Depcrate_builder_scopeimpl_391 {
() => {
// Module: crate::builder::scope
// Provides: {"impl_391"}
// Dependencies: {}
impl < 'tcx > DropTreeBuilder < 'tcx > for Unwind { fn make_block (cfg : & mut CFG < 'tcx >) -> BasicBlock { cfg . start_new_cleanup_block () } fn link_entry_point (cfg : & mut CFG < 'tcx > , from : BasicBlock , to : BasicBlock) { let term = & mut cfg . block_data_mut (from) . terminator_mut () ; match & mut term . kind { TerminatorKind :: Drop { unwind , .. } => { if let UnwindAction :: Cleanup (unwind) = * unwind { let source_info = term . source_info ; cfg . terminate (unwind , source_info , TerminatorKind :: Goto { target : to }) ; } else { * unwind = UnwindAction :: Cleanup (to) ; } } TerminatorKind :: FalseUnwind { unwind , .. } | TerminatorKind :: Call { unwind , .. } | TerminatorKind :: Assert { unwind , .. } | TerminatorKind :: InlineAsm { unwind , .. } => { * unwind = UnwindAction :: Cleanup (to) ; } TerminatorKind :: Goto { .. } | TerminatorKind :: SwitchInt { .. } | TerminatorKind :: UnwindResume | TerminatorKind :: UnwindTerminate (_) | TerminatorKind :: Return | TerminatorKind :: TailCall { .. } | TerminatorKind :: Unreachable | TerminatorKind :: Yield { .. } | TerminatorKind :: CoroutineDrop | TerminatorKind :: FalseEdge { .. } => { span_bug ! (term . source_info . span , "cannot unwind from {:?}" , term . kind) } } } }
};
}
