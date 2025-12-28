macro_rules! deps {
    () => {
        PlaceBuilder!();
        MatchPairTree!();
        Builder!();
        PatternExtraData!();
        FlatPat!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < 'tcx > FlatPat < 'tcx > { # [doc = " Creates a `FlatPat` containing a simplified [`MatchPairTree`] list/forest"] # [doc = " for the given pattern."] fn new (place : PlaceBuilder < 'tcx > , pattern : & Pat < 'tcx > , cx : & mut Builder < '_ , 'tcx >) -> Self { let mut match_pairs = vec ! [] ; let mut extra_data = PatternExtraData { span : pattern . span , bindings : Vec :: new () , ascriptions : Vec :: new () , is_never : pattern . is_never_pattern () , } ; MatchPairTree :: for_pattern (place , pattern , cx , & mut match_pairs , & mut extra_data) ; Self { match_pairs , extra_data } } }
    };
}

impl_111!()