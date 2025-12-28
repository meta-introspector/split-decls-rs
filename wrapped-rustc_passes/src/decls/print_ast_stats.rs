macro_rules! deps {
    () => {
        StatCollector!();
    };
}

macro_rules! print_ast_stats {
    () => {
        deps!();
        pub fn print_ast_stats (tcx : TyCtxt < '_ > , krate : & ast :: Crate) { use rustc_ast :: visit :: Visitor ; let mut collector = StatCollector { tcx : None , nodes : FxHashMap :: default () , seen : FxHashSet :: default () } ; collector . visit_crate (krate) ; collector . print (tcx , "POST EXPANSION AST STATS" , "ast-stats") ; }
    };
}

print_ast_stats!();