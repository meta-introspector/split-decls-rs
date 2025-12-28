macro_rules! deps {
    () => {
        UsefulnessReport!();
        RustcPatCtxt!();
        MatchArm!();
        PatternColumn!();
        PlaceValidity!();
    };
}

macro_rules! analyze_match {
    () => {
        deps!();
        # [doc = " The entrypoint for this crate. Computes whether a match is exhaustive and which of its arms are"] # [doc = " useful, and runs some lints."] pub fn analyze_match < 'p , 'tcx > (tycx : & RustcPatCtxt < 'p , 'tcx > , arms : & [MatchArm < 'p , 'tcx >] , scrut_ty : Ty < 'tcx > ,) -> Result < UsefulnessReport < 'p , 'tcx > , ErrorGuaranteed > { let scrut_ty = tycx . reveal_opaque_ty (scrut_ty) ; let scrut_validity = PlaceValidity :: from_bool (tycx . known_valid_scrutinee) ; let report = compute_match_usefulness (tycx , arms , scrut_ty , scrut_validity , tycx . tcx . pattern_complexity_limit () . 0 ,) ? ; if tycx . refutable && report . non_exhaustiveness_witnesses . is_empty () { let pat_column = PatternColumn :: new (arms) ; lint_nonexhaustive_missing_variants (tycx , arms , & pat_column , scrut_ty) ? ; } Ok (report) }
    };
}

analyze_match!()