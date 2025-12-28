macro_rules! deps {
    () => {
        PatCtxt!();
    };
}

macro_rules! collect_special_tys {
    () => {
        deps!();
        # [doc = " Collect types that require specific explanations when they show up in witnesses."] fn collect_special_tys < 'tcx > (cx : & PatCtxt < '_ , 'tcx > , pat : & WitnessPat < '_ , 'tcx > , special_tys : & mut FxIndexSet < RevealedTy < 'tcx > > ,) { if matches ! (pat . ctor () , Constructor :: NonExhaustive | Constructor :: Never) { special_tys . insert (* pat . ty ()) ; } if let Constructor :: IntRange (range) = pat . ctor () { if cx . is_range_beyond_boundaries (range , * pat . ty ()) { special_tys . insert (* pat . ty ()) ; } } pat . iter_fields () . for_each (| field_pat | collect_special_tys (cx , field_pat , special_tys)) }
    };
}

collect_special_tys!();