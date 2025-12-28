macro_rules! CheckTraitImplStable {
    () => {
        struct CheckTraitImplStable < 'tcx > { tcx : TyCtxt < 'tcx > , fully_stable : bool , }
    };
}

CheckTraitImplStable!()