macro_rules! deps {
    () => {
        UsageMap!();
        PlacedMonoItems!();
        PartitioningCx!();
    };
}

macro_rules! partition {
    () => {
        deps!();
        fn partition < 'tcx , I > (tcx : TyCtxt < 'tcx > , mono_items : I , usage_map : & UsageMap < 'tcx > ,) -> Vec < CodegenUnit < 'tcx > > where I : Iterator < Item = MonoItem < 'tcx > > , { let _prof_timer = tcx . prof . generic_activity ("cgu_partitioning") ; let cx = & PartitioningCx { tcx , usage_map } ; let PlacedMonoItems { mut codegen_units , internalization_candidates } = { let _prof_timer = tcx . prof . generic_activity ("cgu_partitioning_place_items") ; let placed = place_mono_items (cx , mono_items) ; debug_dump (tcx , "PLACE" , & placed . codegen_units) ; placed } ; { let _prof_timer = tcx . prof . generic_activity ("cgu_partitioning_merge_cgus") ; merge_codegen_units (cx , & mut codegen_units) ; debug_dump (tcx , "MERGE" , & codegen_units) ; } if ! tcx . sess . link_dead_code () { let _prof_timer = tcx . prof . generic_activity ("cgu_partitioning_internalize_symbols") ; internalize_symbols (cx , & mut codegen_units , internalization_candidates) ; debug_dump (tcx , "INTERNALIZE" , & codegen_units) ; } if tcx . sess . instrument_coverage () { mark_code_coverage_dead_code_cgu (& mut codegen_units) ; } if ! codegen_units . is_sorted_by (| a , b | a . name () . as_str () <= b . name () . as_str ()) { let mut names = String :: new () ; for cgu in codegen_units . iter () { names += & format ! ("- {}\n" , cgu . name ()) ; } bug ! ("unsorted CGUs:\n{names}") ; } codegen_units }
    };
}

partition!();