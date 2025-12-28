macro_rules! deps {
    () => {
        ComesFromAllowExpect!();
    };
}

macro_rules! create_and_seed_worklist {
    () => {
        deps!();
        fn create_and_seed_worklist (tcx : TyCtxt < '_ > ,) -> (Vec < (LocalDefId , ComesFromAllowExpect) > , Vec < LocalDefId >) { let effective_visibilities = & tcx . effective_visibilities (()) ; let mut unsolved_impl_item = Vec :: new () ; let mut worklist = effective_visibilities . iter () . filter_map (| (& id , effective_vis) | { effective_vis . is_public_at_level (Level :: Reachable) . then_some (id) . map (| id | (id , ComesFromAllowExpect :: No)) }) . chain (tcx . entry_fn (()) . and_then (| (def_id , _) | def_id . as_local () . map (| id | (id , ComesFromAllowExpect :: No))) ,) . collect :: < Vec < _ > > () ; let crate_items = tcx . hir_crate_items (()) ; for id in crate_items . owners () { maybe_record_as_seed (tcx , id , & mut worklist , & mut unsolved_impl_item) ; } (worklist , unsolved_impl_item) }
    };
}

create_and_seed_worklist!()