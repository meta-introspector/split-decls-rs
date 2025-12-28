macro_rules! create_query_frame_extra {
    () => {
        fn create_query_frame_extra < 'tcx , K : Key + Copy + 'tcx > ((tcx , key , kind , name , do_describe) : (TyCtxt < 'tcx > , K , DepKind , & 'static str , fn (TyCtxt < 'tcx > , K) -> String ,) ,) -> QueryStackFrameExtra { let def_id = key . key_as_def_id () ; let reduce_queries = with_reduced_queries () ; let description = ty :: print :: with_no_queries ! (do_describe (tcx , key)) ; let description = if tcx . sess . verbose_internals () { format ! ("{description} [{name:?}]") } else { description } ; let span = if kind == dep_graph :: dep_kinds :: def_span || reduce_queries { None } else { Some (key . default_span (tcx)) } ; let def_kind = if kind == dep_graph :: dep_kinds :: def_kind || reduce_queries { None } else { def_id . and_then (| def_id | def_id . as_local ()) . map (| def_id | tcx . def_kind (def_id)) } ; QueryStackFrameExtra :: new (description , span , def_kind) }
    };
}

create_query_frame_extra!()