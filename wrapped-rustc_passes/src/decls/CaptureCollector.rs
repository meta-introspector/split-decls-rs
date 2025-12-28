macro_rules! CaptureCollector {
    () => {
        struct CaptureCollector < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , locals : & 'a FxHashSet < HirId > , upvars : FxIndexMap < HirId , hir :: Upvar > , }
    };
}

CaptureCollector!()