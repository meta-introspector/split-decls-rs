macro_rules! deps {
    () => {
        NonExportedMacroInvalidAttrs!();
    };
}

macro_rules! check_non_exported_macro_for_invalid_attrs {
    () => {
        deps!();
        fn check_non_exported_macro_for_invalid_attrs (tcx : TyCtxt < '_ > , item : & Item < '_ >) { let attrs = tcx . hir_attrs (item . hir_id ()) ; if let Some (attr_span) = find_attr ! (attrs , AttributeKind :: Inline (i , span) if ! matches ! (i , InlineAttr :: Force { .. }) => * span) { tcx . dcx () . emit_err (errors :: NonExportedMacroInvalidAttrs { attr_span }) ; } }
    };
}

check_non_exported_macro_for_invalid_attrs!();