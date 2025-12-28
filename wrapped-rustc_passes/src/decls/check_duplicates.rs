macro_rules! deps {
    () => {
        UnusedMultiple!();
        UnusedDuplicate!();
    };
}

macro_rules! check_duplicates {
    () => {
        deps!();
        fn check_duplicates (tcx : TyCtxt < '_ > , attr : & Attribute , hir_id : HirId , duplicates : AttributeDuplicates , seen : & mut FxHashMap < Symbol , Span > ,) { use AttributeDuplicates :: * ; if matches ! (duplicates , WarnFollowingWordOnly) && ! attr . is_word () { return ; } let attr_name = attr . name () . unwrap () ; match duplicates { DuplicatesOk => { } WarnFollowing | FutureWarnFollowing | WarnFollowingWordOnly | FutureWarnPreceding => { match seen . entry (attr_name) { Entry :: Occupied (mut entry) => { let (this , other) = if matches ! (duplicates , FutureWarnPreceding) { let to_remove = entry . insert (attr . span ()) ; (to_remove , attr . span ()) } else { (attr . span () , * entry . get ()) } ; tcx . emit_node_span_lint (UNUSED_ATTRIBUTES , hir_id , this , errors :: UnusedDuplicate { this , other , warning : matches ! (duplicates , FutureWarnFollowing | FutureWarnPreceding) , } ,) ; } Entry :: Vacant (entry) => { entry . insert (attr . span ()) ; } } } ErrorFollowing | ErrorPreceding => match seen . entry (attr_name) { Entry :: Occupied (mut entry) => { let (this , other) = if matches ! (duplicates , ErrorPreceding) { let to_remove = entry . insert (attr . span ()) ; (to_remove , attr . span ()) } else { (attr . span () , * entry . get ()) } ; tcx . dcx () . emit_err (errors :: UnusedMultiple { this , other , name : attr_name }) ; } Entry :: Vacant (entry) => { entry . insert (attr . span ()) ; } } , } }
    };
}

check_duplicates!()