// Generated macro for impl_389 (impl)
macro_rules! Depcrate_builder_scopeimpl_389 {
() => {
// Module: crate::builder::scope
// Provides: {"impl_389"}
// Dependencies: {}
impl < 'tcx > DropTreeBuilder < 'tcx > for CoroutineDrop { fn make_block (cfg : & mut CFG < 'tcx >) -> BasicBlock { cfg . start_new_block () } fn link_entry_point (cfg : & mut CFG < 'tcx > , from : BasicBlock , to : BasicBlock) { let term = cfg . block_data_mut (from) . terminator_mut () ; if let TerminatorKind :: Yield { ref mut drop , .. } = term . kind { * drop = Some (to) ; } else if let TerminatorKind :: Drop { ref mut drop , .. } = term . kind { * drop = Some (to) ; } else { span_bug ! (term . source_info . span , "cannot enter coroutine drop tree from {:?}" , term . kind) } } }
};
}
