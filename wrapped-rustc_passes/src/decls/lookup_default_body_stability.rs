macro_rules! lookup_default_body_stability {
    () => {
        # [instrument (level = "debug" , skip (tcx))] fn lookup_default_body_stability (tcx : TyCtxt < '_ > , def_id : LocalDefId ,) -> Option < DefaultBodyStability > { if ! tcx . features () . staged_api () { return None ; } let attrs = tcx . hir_attrs (tcx . local_def_id_to_hir_id (def_id)) ; find_attr ! (attrs , AttributeKind :: BodyStability { stability , .. } => * stability) }
    };
}

lookup_default_body_stability!()