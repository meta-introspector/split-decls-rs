macro_rules! NonExhaustiveMatchAllArmsGuarded {
    () => {
        # [derive (Subdiagnostic)] # [note (mir_build_non_exhaustive_match_all_arms_guarded)] pub (crate) struct NonExhaustiveMatchAllArmsGuarded ;
    };
}

NonExhaustiveMatchAllArmsGuarded!();