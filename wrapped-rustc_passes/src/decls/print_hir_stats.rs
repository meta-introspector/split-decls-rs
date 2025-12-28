macro_rules! deps {
    () => {
        StatCollector!();
    };
}

macro_rules! print_hir_stats {
    () => {
        deps!();
        pub fn print_hir_stats (tcx : TyCtxt < '_ >) { let mut collector = StatCollector { tcx : Some (tcx) , nodes : FxHashMap :: default () , seen : FxHashSet :: default () } ; tcx . hir_walk_toplevel_module (& mut collector) ; tcx . hir_walk_attributes (& mut collector) ; collector . print (tcx , "HIR STATS" , "hir-stats") ; }
    };
}

print_hir_stats!()