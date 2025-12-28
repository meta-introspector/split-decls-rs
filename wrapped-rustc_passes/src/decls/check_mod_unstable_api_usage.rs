macro_rules! deps {
    () => {
        Checker!();
        MissingStabilityAnnotations!();
    };
}

macro_rules! check_mod_unstable_api_usage {
    () => {
        deps!();
        # [doc = " Cross-references the feature names of unstable APIs with enabled"] # [doc = " features and possibly prints errors."] fn check_mod_unstable_api_usage (tcx : TyCtxt < '_ > , module_def_id : LocalModDefId) { tcx . hir_visit_item_likes_in_module (module_def_id , & mut Checker { tcx }) ; let is_staged_api = tcx . sess . opts . unstable_opts . force_unstable_if_unmarked || tcx . features () . staged_api () ; if is_staged_api { let effective_visibilities = & tcx . effective_visibilities (()) ; let mut missing = MissingStabilityAnnotations { tcx , effective_visibilities } ; if module_def_id . is_top_level_module () { missing . check_missing_stability (CRATE_DEF_ID) ; } tcx . hir_visit_item_likes_in_module (module_def_id , & mut missing) ; } if module_def_id . is_top_level_module () { check_unused_or_stable_features (tcx) } }
    };
}

check_mod_unstable_api_usage!()