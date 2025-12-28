macro_rules! ReachableContext {
    () => {
        struct ReachableContext < 'tcx > { tcx : TyCtxt < 'tcx > , maybe_typeck_results : Option < & 'tcx ty :: TypeckResults < 'tcx > > , reachable_symbols : LocalDefIdSet , worklist : Vec < LocalDefId > , any_library : bool , }
    };
}

ReachableContext!()