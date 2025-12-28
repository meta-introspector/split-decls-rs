macro_rules! ImplsOrderVisitor {
    () => {
        struct ImplsOrderVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , order : FxIndexMap < DefId , usize > , }
    };
}

ImplsOrderVisitor!();