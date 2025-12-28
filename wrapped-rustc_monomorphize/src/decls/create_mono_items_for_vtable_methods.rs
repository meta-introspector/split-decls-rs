macro_rules! deps {
    () => {
        MonoItems!();
    };
}

macro_rules! create_mono_items_for_vtable_methods {
    () => {
        deps!();
        # [doc = " Creates a `MonoItem` for each method that is referenced by the vtable for"] # [doc = " the given trait/impl pair."] fn create_mono_items_for_vtable_methods < 'tcx > (tcx : TyCtxt < 'tcx > , trait_ty : Ty < 'tcx > , impl_ty : Ty < 'tcx > , source : Span , output : & mut MonoItems < 'tcx > ,) { assert ! (! trait_ty . has_escaping_bound_vars () && ! impl_ty . has_escaping_bound_vars ()) ; let ty :: Dynamic (trait_ty , ..) = trait_ty . kind () else { bug ! ("create_mono_items_for_vtable_methods: {trait_ty:?} not a trait type") ; } ; if let Some (principal) = trait_ty . principal () { let trait_ref = tcx . instantiate_bound_regions_with_erased (principal . with_self_ty (tcx , impl_ty)) ; assert ! (! trait_ref . has_escaping_bound_vars ()) ; let entries = tcx . vtable_entries (trait_ref) ; debug ! (? entries) ; let methods = entries . iter () . filter_map (| entry | match entry { VtblEntry :: MetadataDropInPlace | VtblEntry :: MetadataSize | VtblEntry :: MetadataAlign | VtblEntry :: Vacant => None , VtblEntry :: TraitVPtr (_) => { None } VtblEntry :: Method (instance) => { Some (* instance) . filter (| instance | tcx . should_codegen_locally (* instance)) } }) . map (| item | create_fn_mono_item (tcx , item , source)) ; output . extend (methods) ; } if impl_ty . needs_drop (tcx , ty :: TypingEnv :: fully_monomorphized ()) { visit_drop_use (tcx , impl_ty , false , source , output) ; } }
    };
}

create_mono_items_for_vtable_methods!();