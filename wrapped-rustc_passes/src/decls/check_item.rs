macro_rules! check_item {
    () => {
        fn check_item < 'tcx > (tcx : TyCtxt < 'tcx > , id : hir :: ItemId , worklist : & mut Vec < LocalDefId > , effective_visibilities : & privacy :: EffectiveVisibilities ,) { if has_custom_linkage (tcx , id . owner_id . def_id) { worklist . push (id . owner_id . def_id) ; } if ! matches ! (tcx . def_kind (id . owner_id) , DefKind :: Impl { of_trait : true }) { return ; } if effective_visibilities . is_reachable (id . owner_id . def_id) { return ; } let items = tcx . associated_item_def_ids (id . owner_id) ; worklist . extend (items . iter () . map (| ii_ref | ii_ref . expect_local ())) ; let Some (trait_def_id) = tcx . trait_id_of_impl (id . owner_id . to_def_id ()) else { unreachable ! () ; } ; if ! trait_def_id . is_local () { return ; } worklist . extend (tcx . provided_trait_methods (trait_def_id) . map (| assoc | assoc . def_id . expect_local ())) ; }
    };
}

check_item!();