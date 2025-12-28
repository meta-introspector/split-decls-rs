macro_rules! deps {
    () => {
        ItemFollowingInnerAttr!();
        InvalidAttrAtCrateLevel!();
    };
}

macro_rules! check_invalid_crate_level_attr {
    () => {
        deps!();
        fn check_invalid_crate_level_attr (tcx : TyCtxt < '_ > , attrs : & [Attribute]) { const ATTRS_TO_CHECK : & [Symbol] = & [sym :: macro_export , sym :: rustc_main , sym :: derive , sym :: test , sym :: test_case , sym :: global_allocator , sym :: bench ,] ; for attr in attrs { let (span , name) = if let Some (a) = ATTRS_TO_CHECK . iter () . find (| attr_to_check | attr . has_name (* * attr_to_check)) { (attr . span () , * a) } else if let Attribute :: Parsed (AttributeKind :: Repr { reprs : _ , first_span : first_attr_span , }) = attr { (* first_attr_span , sym :: repr) } else { continue ; } ; let item = tcx . hir_free_items () . map (| id | tcx . hir_item (id)) . find (| item | ! item . span . is_dummy ()) . map (| item | errors :: ItemFollowingInnerAttr { span : if let Some (ident) = item . kind . ident () { ident . span } else { item . span } , kind : tcx . def_descr (item . owner_id . to_def_id ()) , }) ; let err = tcx . dcx () . create_err (errors :: InvalidAttrAtCrateLevel { span , sugg_span : tcx . sess . source_map () . span_to_snippet (span) . ok () . filter (| src | src . starts_with ("#![")) . map (| _ | span . with_lo (span . lo () + BytePos (1)) . with_hi (span . lo () + BytePos (2))) , name , item , }) ; if let Attribute :: Unparsed (p) = attr { tcx . dcx () . try_steal_replace_and_emit_err (p . path . span , StashKey :: UndeterminedMacroResolution , err ,) ; } else { err . emit () ; } } }
    };
}

check_invalid_crate_level_attr!()