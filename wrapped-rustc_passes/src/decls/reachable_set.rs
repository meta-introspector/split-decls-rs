macro_rules! deps {
    () => {
        ReachableContext!();
    };
}

macro_rules! reachable_set {
    () => {
        deps!();
        # [doc = " See module-level doc comment above."] fn reachable_set (tcx : TyCtxt < '_ > , () : ()) -> LocalDefIdSet { let effective_visibilities = & tcx . effective_visibilities (()) ; let any_library = tcx . crate_types () . iter () . any (| ty | { * ty == CrateType :: Rlib || * ty == CrateType :: Dylib || * ty == CrateType :: ProcMacro || * ty == CrateType :: Sdylib }) ; let mut reachable_context = ReachableContext { tcx , maybe_typeck_results : None , reachable_symbols : Default :: default () , worklist : Vec :: new () , any_library , } ; reachable_context . worklist = effective_visibilities . iter () . filter_map (| (& id , effective_vis) | { effective_vis . is_public_at_level (Level :: ReachableThroughImplTrait) . then_some (id) }) . collect :: < Vec < _ > > () ; for (_ , def_id) in tcx . lang_items () . iter () { if let Some (def_id) = def_id . as_local () { reachable_context . worklist . push (def_id) ; } } { let crate_items = tcx . hir_crate_items (()) ; for id in crate_items . free_items () { check_item (tcx , id , & mut reachable_context . worklist , effective_visibilities) ; } for id in crate_items . impl_items () { if has_custom_linkage (tcx , id . owner_id . def_id) { reachable_context . worklist . push (id . owner_id . def_id) ; } } } reachable_context . propagate () ; debug ! ("Inline reachability shows: {:?}" , reachable_context . reachable_symbols) ; reachable_context . reachable_symbols }
    };
}

reachable_set!();