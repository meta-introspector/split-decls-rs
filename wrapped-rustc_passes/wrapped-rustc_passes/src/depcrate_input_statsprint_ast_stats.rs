// Generated macro for print_ast_stats (function)
macro_rules! Depcrate_input_statsprint_ast_stats {
() => {
// Module: crate::input_stats
// Provides: {"print_ast_stats"}
// Dependencies: {}
pub fn print_ast_stats (tcx : TyCtxt < '_ > , krate : & ast :: Crate) { use rustc_ast :: visit :: Visitor ; let mut collector = StatCollector { tcx : None , nodes : FxHashMap :: default () , seen : FxHashSet :: default () } ; collector . visit_crate (krate) ; collector . print (tcx , "POST EXPANSION AST STATS" , "ast-stats") ; }
};
}
