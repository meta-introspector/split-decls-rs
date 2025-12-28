macro_rules! deps {
    () => {
        ExportableItemCollector!();
        UnexportableItem!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'tcx > ExportableItemCollector < 'tcx > { fn new (tcx : TyCtxt < 'tcx >) -> ExportableItemCollector < 'tcx > { ExportableItemCollector { tcx , exportable_items : Default :: default () , in_exportable_mod : false , seen_exportable_in_mod : false , } } fn report_wrong_site (& self , def_id : LocalDefId) { let def_descr = self . tcx . def_descr (def_id . to_def_id ()) ; self . tcx . dcx () . emit_err (UnexportableItem :: Item { descr : & format ! ("{}" , def_descr) , span : self . tcx . def_span (def_id) , }) ; } fn item_is_exportable (& self , def_id : LocalDefId) -> bool { let has_attr = find_attr ! (self . tcx . get_all_attrs (def_id) , AttributeKind :: ExportStable) ; if ! self . in_exportable_mod && ! has_attr { return false ; } let visibilities = self . tcx . effective_visibilities (()) ; let is_pub = visibilities . is_directly_public (def_id) ; if has_attr && ! is_pub { let vis = visibilities . effective_vis (def_id) . cloned () . unwrap_or_else (| | { EffectiveVisibility :: from_vis (Visibility :: Restricted (self . tcx . parent_module_from_def_id (def_id) . to_local_def_id () ,)) }) ; let vis = vis . at_level (Level :: Direct) ; let span = self . tcx . def_span (def_id) ; self . tcx . dcx () . emit_err (UnexportableItem :: PrivItem { vis_note : span , vis_descr : & vis . to_string (def_id , self . tcx) , span , }) ; return false ; } is_pub && (has_attr || self . in_exportable_mod) } fn add_exportable (& mut self , def_id : LocalDefId) { self . seen_exportable_in_mod = true ; self . exportable_items . insert (def_id . to_def_id ()) ; } fn walk_item_with_mod (& mut self , item : & 'tcx hir :: Item < 'tcx >) { let def_id = item . hir_id () . owner . def_id ; let old_exportable_mod = self . in_exportable_mod ; if find_attr ! (self . tcx . get_all_attrs (def_id) , AttributeKind :: ExportStable) { self . in_exportable_mod = true ; } let old_seen_exportable_in_mod = std :: mem :: replace (& mut self . seen_exportable_in_mod , false) ; intravisit :: walk_item (self , item) ; if self . seen_exportable_in_mod || self . in_exportable_mod { self . exportable_items . insert (def_id . to_def_id ()) ; } self . seen_exportable_in_mod = old_seen_exportable_in_mod ; self . in_exportable_mod = old_exportable_mod ; } }
    };
}

impl_25!();