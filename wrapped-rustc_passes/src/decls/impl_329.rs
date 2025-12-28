macro_rules! deps {
    () => {
        MissingStabilityAnnotations!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for MissingStabilityAnnotations < 'tcx > { type NestedFilter = nested_filter :: OnlyBodies ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_item (& mut self , i : & 'tcx Item < 'tcx >) { self . check_compatible_stability (i . owner_id . def_id) ; if ! matches ! (i . kind , hir :: ItemKind :: Impl (hir :: Impl { of_trait : None , .. }) | hir :: ItemKind :: ForeignMod { .. }) { self . check_missing_stability (i . owner_id . def_id) ; } self . check_missing_const_stability (i . owner_id . def_id) ; intravisit :: walk_item (self , i) } fn visit_trait_item (& mut self , ti : & 'tcx hir :: TraitItem < 'tcx >) { self . check_compatible_stability (ti . owner_id . def_id) ; self . check_missing_stability (ti . owner_id . def_id) ; intravisit :: walk_trait_item (self , ti) ; } fn visit_impl_item (& mut self , ii : & 'tcx hir :: ImplItem < 'tcx >) { self . check_compatible_stability (ii . owner_id . def_id) ; if let hir :: ImplItemImplKind :: Inherent { .. } = ii . impl_kind { self . check_missing_stability (ii . owner_id . def_id) ; self . check_missing_const_stability (ii . owner_id . def_id) ; } intravisit :: walk_impl_item (self , ii) ; } fn visit_variant (& mut self , var : & 'tcx Variant < 'tcx >) { self . check_compatible_stability (var . def_id) ; self . check_missing_stability (var . def_id) ; if let Some (ctor_def_id) = var . data . ctor_def_id () { self . check_missing_stability (ctor_def_id) ; } intravisit :: walk_variant (self , var) ; } fn visit_field_def (& mut self , s : & 'tcx FieldDef < 'tcx >) { self . check_compatible_stability (s . def_id) ; self . check_missing_stability (s . def_id) ; intravisit :: walk_field_def (self , s) ; } fn visit_foreign_item (& mut self , i : & 'tcx hir :: ForeignItem < 'tcx >) { self . check_compatible_stability (i . owner_id . def_id) ; self . check_missing_stability (i . owner_id . def_id) ; intravisit :: walk_foreign_item (self , i) ; } fn visit_generic_param (& mut self , p : & 'tcx hir :: GenericParam < 'tcx >) { self . check_compatible_stability (p . def_id) ; intravisit :: walk_generic_param (self , p) ; } }
    };
}

impl_329!()