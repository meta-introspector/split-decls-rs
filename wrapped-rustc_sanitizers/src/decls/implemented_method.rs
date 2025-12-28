macro_rules! implemented_method {
    () => {
        fn implemented_method < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > ,) -> Option < (ty :: EarlyBinder < 'tcx , TraitRef < 'tcx > > , DefId , DefId) > { let trait_ref ; let method_id ; let trait_id ; let trait_method ; let ancestor = if let Some (impl_id) = tcx . impl_of_assoc (instance . def_id ()) { trait_ref = tcx . impl_trait_ref (impl_id) ? ; method_id = tcx . trait_item_of (instance . def_id ()) ? ; trait_method = tcx . associated_item (method_id) ; trait_id = trait_ref . skip_binder () . def_id ; impl_id } else if let InstanceKind :: Item (def_id) = instance . def && let Some (trait_method_bound) = tcx . opt_associated_item (def_id) { trait_method = trait_method_bound ; method_id = instance . def_id () ; trait_id = tcx . trait_of_assoc (method_id) ? ; trait_ref = ty :: EarlyBinder :: bind (TraitRef :: from_assoc (tcx , trait_id , instance . args)) ; trait_id } else { return None ; } ; let vtable_possible = traits :: is_vtable_safe_method (tcx , trait_id , trait_method) && tcx . is_dyn_compatible (trait_id) ; vtable_possible . then_some ((trait_ref , method_id , ancestor)) }
    };
}

implemented_method!()