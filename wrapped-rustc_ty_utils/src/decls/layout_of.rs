macro_rules! layout_of {
    () => {
        # [instrument (skip (tcx , query) , level = "debug")] fn layout_of < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> Result < TyAndLayout < 'tcx > , & 'tcx LayoutError < 'tcx > > { let PseudoCanonicalInput { typing_env , value : ty } = query ; debug ! (? ty) ; let typing_env = typing_env . with_post_analysis_normalized (tcx) ; let unnormalized_ty = ty ; let ty = match tcx . try_normalize_erasing_regions (typing_env , ty) { Ok (t) => t , Err (normalization_error) => { return Err (tcx . arena . alloc (LayoutError :: NormalizationFailure (ty , normalization_error))) ; } } ; if ty != unnormalized_ty { return tcx . layout_of (typing_env . as_query_input (ty)) ; } let cx = LayoutCx :: new (tcx , typing_env) ; let layout = layout_of_uncached (& cx , ty) ? ; let layout = TyAndLayout { ty , layout } ; if cx . tcx () . sess . opts . unstable_opts . print_type_sizes { record_layout_for_printing (& cx , layout) ; } invariant :: layout_sanity_check (& cx , & layout) ; Ok (layout) }
    };
}

layout_of!();