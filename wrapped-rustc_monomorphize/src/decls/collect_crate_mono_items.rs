macro_rules! deps {
    () => {
        MonoItemCollectionStrategy!();
        UsageMap!();
        SharedState!();
    };
}

macro_rules! collect_crate_mono_items {
    () => {
        deps!();
        # [instrument (skip (tcx , strategy) , level = "debug")] pub (crate) fn collect_crate_mono_items < 'tcx > (tcx : TyCtxt < 'tcx > , strategy : MonoItemCollectionStrategy ,) -> (Vec < MonoItem < 'tcx > > , UsageMap < 'tcx >) { let _prof_timer = tcx . prof . generic_activity ("monomorphization_collector") ; let roots = tcx . sess . time ("monomorphization_collector_root_collections" , | | collect_roots (tcx , strategy)) ; debug ! ("building mono item graph, beginning at roots") ; let state = SharedState { visited : MTLock :: new (UnordSet :: default ()) , mentioned : MTLock :: new (UnordSet :: default ()) , usage_map : MTLock :: new (UsageMap :: new ()) , } ; let recursion_limit = tcx . recursion_limit () ; tcx . sess . time ("monomorphization_collector_graph_walk" , | | { par_for_each_in (roots , | root | { collect_items_root (tcx , dummy_spanned (* root) , & state , recursion_limit) ; }) ; }) ; let mono_items = tcx . with_stable_hashing_context (move | ref hcx | { state . visited . into_inner () . into_sorted (hcx , true) }) ; (mono_items , state . usage_map . into_inner ()) }
    };
}

collect_crate_mono_items!();