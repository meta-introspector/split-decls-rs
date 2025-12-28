macro_rules! deps {
    () => {
        ShouldWarnAboutField!();
        ReportOn!();
        DeadItem!();
        DeadVisitor!();
    };
}

macro_rules! check_mod_deathness {
    () => {
        deps!();
        fn check_mod_deathness (tcx : TyCtxt < '_ > , module : LocalModDefId) { let (live_symbols , ignored_derived_traits) = tcx . live_symbols_and_ignored_derived_traits (()) ; let mut visitor = DeadVisitor { tcx , live_symbols , ignored_derived_traits } ; let module_items = tcx . hir_module_items (module) ; for item in module_items . free_items () { let def_kind = tcx . def_kind (item . owner_id) ; let mut dead_codes = Vec :: new () ; if matches ! (def_kind , DefKind :: Impl { of_trait : false }) || (def_kind == DefKind :: Trait && live_symbols . contains (& item . owner_id . def_id)) { for & def_id in tcx . associated_item_def_ids (item . owner_id . def_id) { if let Some (local_def_id) = def_id . as_local () && ! visitor . is_live_code (local_def_id) { let name = tcx . item_name (def_id) ; let level = visitor . def_lint_level (local_def_id) ; dead_codes . push (DeadItem { def_id : local_def_id , name , level }) ; } } } if ! dead_codes . is_empty () { visitor . warn_multiple (item . owner_id . def_id , "used" , dead_codes , ReportOn :: NamedField) ; } if ! live_symbols . contains (& item . owner_id . def_id) { let parent = tcx . local_parent (item . owner_id . def_id) ; if parent != module . to_local_def_id () && ! live_symbols . contains (& parent) { continue ; } visitor . check_definition (item . owner_id . def_id) ; continue ; } if let DefKind :: Struct | DefKind :: Union | DefKind :: Enum = def_kind { let adt = tcx . adt_def (item . owner_id) ; let mut dead_variants = Vec :: new () ; for variant in adt . variants () { let def_id = variant . def_id . expect_local () ; if ! live_symbols . contains (& def_id) { let level = visitor . def_lint_level (def_id) ; dead_variants . push (DeadItem { def_id , name : variant . name , level }) ; continue ; } let is_positional = variant . fields . raw . first () . is_some_and (| field | { field . name . as_str () . starts_with (| c : char | c . is_ascii_digit ()) }) ; let report_on = if is_positional { ReportOn :: TupleField } else { ReportOn :: NamedField } ; let dead_fields = variant . fields . iter () . filter_map (| field | { let def_id = field . did . expect_local () ; if let ShouldWarnAboutField :: Yes = visitor . should_warn_about_field (field) { let level = visitor . def_lint_level (def_id) ; Some (DeadItem { def_id , name : field . name , level }) } else { None } }) . collect () ; visitor . warn_multiple (def_id , "read" , dead_fields , report_on) ; } visitor . warn_multiple (item . owner_id . def_id , "constructed" , dead_variants , ReportOn :: NamedField ,) ; } } for foreign_item in module_items . foreign_items () { visitor . check_definition (foreign_item . owner_id . def_id) ; } }
    };
}

check_mod_deathness!()