macro_rules! SearchInterfaceForPrivateItemsVisitor {
    () => {
        # [doc = " SearchInterfaceForPrivateItemsVisitor traverses an item's interface and"] # [doc = " finds any private components in it."] # [doc = " PrivateItemsInPublicInterfacesVisitor ensures there are no private types"] # [doc = " and traits in public interfaces."] struct SearchInterfaceForPrivateItemsVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , item_def_id : LocalDefId , # [doc = " The visitor checks that each component type is at least this visible."] required_visibility : ty :: Visibility , required_effective_vis : Option < EffectiveVisibility > , in_assoc_ty : bool , in_primary_interface : bool , skip_assoc_tys : bool , }
    };
}

SearchInterfaceForPrivateItemsVisitor!();