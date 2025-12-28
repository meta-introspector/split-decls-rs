macro_rules! deps {
    () => {
        OpaqueTypeCollector!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for OpaqueTypeCollector < 'tcx > { # [instrument (skip (self) , ret , level = "trace")] fn visit_ty (& mut self , t : Ty < 'tcx >) { t . super_visit_with (self) ; match * t . kind () { ty :: Alias (ty :: Opaque , alias_ty) if alias_ty . def_id . is_local () => { self . visit_opaque_ty (alias_ty) ; } ty :: Alias (ty :: Free , alias_ty) if let Some (def_id) = alias_ty . def_id . as_local () => { if ! self . seen . insert (def_id) { return ; } self . tcx . type_of (alias_ty . def_id) . instantiate (self . tcx , alias_ty . args) . visit_with (self) ; } ty :: Alias (ty :: Projection , alias_ty) => { if let Some (impl_trait_ref) = self . parent_impl_trait_ref () { if alias_ty . trait_ref (self . tcx) == impl_trait_ref { let parent = self . parent () . expect ("we should have a parent here") ; for & assoc in self . tcx . associated_items (parent) . in_definition_order () { trace ! (? assoc) ; if assoc . expect_trait_impl () != Ok (alias_ty . def_id) { continue ; } if ! assoc . defaultness (self . tcx) . is_final () { continue ; } if ! self . seen . insert (assoc . def_id . expect_local ()) { return ; } let alias_args = alias_ty . args . rebase_onto (self . tcx , impl_trait_ref . def_id , ty :: GenericArgs :: identity_for_item (self . tcx , parent) ,) ; if self . tcx . check_args_compatible (assoc . def_id , alias_args) { self . tcx . type_of (assoc . def_id) . instantiate (self . tcx , alias_args) . visit_with (self) ; return ; } else { self . tcx . dcx () . span_delayed_bug (self . tcx . def_span (assoc . def_id) , "item had incorrect args" ,) ; } } } } else if let Some (ty :: ImplTraitInTraitData :: Trait { fn_def_id , .. }) = self . tcx . opt_rpitit_info (alias_ty . def_id) && fn_def_id == self . item . into () { let ty = self . tcx . type_of (alias_ty . def_id) . instantiate (self . tcx , alias_ty . args) ; let ty :: Alias (ty :: Opaque , alias_ty) = * ty . kind () else { bug ! ("{ty:?}") } ; self . visit_opaque_ty (alias_ty) ; } } _ => trace ! (kind =? t . kind ()) , } } }
    };
}

impl_102!();