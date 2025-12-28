macro_rules! DeadVisitor {
    () => {
        struct DeadVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , live_symbols : & 'tcx LocalDefIdSet , ignored_derived_traits : & 'tcx LocalDefIdMap < FxIndexSet < DefId > > , }
    };
}

DeadVisitor!()