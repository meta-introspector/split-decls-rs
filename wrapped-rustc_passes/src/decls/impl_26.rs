macro_rules! deps {
    () => {
        ExportableItemCollector!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for ExportableItemCollector < 'tcx > { type NestedFilter = nested_filter :: All ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_item (& mut self , item : & 'tcx hir :: Item < 'tcx >) { let def_id = item . hir_id () . owner . def_id ; match item . kind { hir :: ItemKind :: Mod (..) => { self . walk_item_with_mod (item) ; return ; } hir :: ItemKind :: Impl (impl_) if impl_ . of_trait . is_none () => { self . walk_item_with_mod (item) ; return ; } _ => { } } if ! self . item_is_exportable (def_id) { return ; } match item . kind { hir :: ItemKind :: Fn { .. } | hir :: ItemKind :: Struct (..) | hir :: ItemKind :: Enum (..) | hir :: ItemKind :: Union (..) | hir :: ItemKind :: TyAlias (..) => { self . add_exportable (def_id) ; } hir :: ItemKind :: Use (path , _) => { for res in path . res . present_items () { if let Some (res_id) = res . opt_def_id () && let Some (res_id) = res_id . as_local () { self . add_exportable (res_id) ; } } } hir :: ItemKind :: Mod (..) => unreachable ! () , hir :: ItemKind :: Impl (impl_) if impl_ . of_trait . is_none () => { unreachable ! () ; } _ => self . report_wrong_site (def_id) , } } fn visit_impl_item (& mut self , item : & 'tcx hir :: ImplItem < 'tcx >) { let def_id = item . hir_id () . owner . def_id ; if ! self . item_is_exportable (def_id) { return ; } match item . kind { hir :: ImplItemKind :: Fn (..) | hir :: ImplItemKind :: Type (..) => { self . add_exportable (def_id) ; } _ => self . report_wrong_site (def_id) , } } fn visit_foreign_item (& mut self , item : & 'tcx hir :: ForeignItem < 'tcx >) { let def_id = item . hir_id () . owner . def_id ; if ! self . item_is_exportable (def_id) { self . report_wrong_site (def_id) ; } } fn visit_trait_item (& mut self , item : & 'tcx hir :: TraitItem < 'tcx >) { let def_id = item . hir_id () . owner . def_id ; if ! self . item_is_exportable (def_id) { self . report_wrong_site (def_id) ; } } }
    };
}

impl_26!();