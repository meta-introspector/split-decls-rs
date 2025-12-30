// Generated macro for print_hir_stats (function)
macro_rules! Depcrate_input_statsprint_hir_stats {
() => {
// Module: crate::input_stats
// Provides: {"print_hir_stats"}
// Dependencies: {}
pub fn print_hir_stats (tcx : TyCtxt < '_ >) { let mut collector = StatCollector { tcx : Some (tcx) , nodes : FxHashMap :: default () , seen : FxHashSet :: default () } ; tcx . hir_walk_toplevel_module (& mut collector) ; tcx . hir_walk_attributes (& mut collector) ; collector . print (tcx , "HIR STATS" , "hir-stats") ; }
};
}
