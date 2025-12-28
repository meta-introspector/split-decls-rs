macro_rules! lookup_deprecation_entry {
    () => {
        fn lookup_deprecation_entry (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Option < DeprecationEntry > { let attrs = tcx . hir_attrs (tcx . local_def_id_to_hir_id (def_id)) ; let depr = find_attr ! (attrs , AttributeKind :: Deprecation { deprecation , span : _ } => * deprecation) ; let Some (depr) = depr else { if inherit_deprecation (tcx . def_kind (def_id)) { let parent_id = tcx . opt_local_parent (def_id) ? ; let parent_depr = tcx . lookup_deprecation_entry (parent_id) ? ; return Some (parent_depr) ; } return None ; } ; Some (DeprecationEntry :: local (depr , def_id)) }
    };
}

lookup_deprecation_entry!();