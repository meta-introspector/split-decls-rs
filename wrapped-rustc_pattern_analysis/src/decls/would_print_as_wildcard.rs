macro_rules! deps {
    () => {
        IntRange!();
        Constructor!();
        WitnessPat!();
        MaybeInfiniteInt!();
    };
}

macro_rules! would_print_as_wildcard {
    () => {
        deps!();
        # [doc = " Returns `true` if the given pattern would be printed as a wildcard (`_`)."] fn would_print_as_wildcard (tcx : TyCtxt < '_ > , p : & WitnessPat < '_ , '_ >) -> bool { match p . ctor () { Constructor :: IntRange (IntRange { lo : MaybeInfiniteInt :: NegInfinity , hi : MaybeInfiniteInt :: PosInfinity , }) | Constructor :: Wildcard | Constructor :: NonExhaustive | Constructor :: Hidden | Constructor :: PrivateUninhabited => true , Constructor :: Never if ! tcx . features () . never_patterns () => true , _ => false , } }
    };
}

would_print_as_wildcard!()