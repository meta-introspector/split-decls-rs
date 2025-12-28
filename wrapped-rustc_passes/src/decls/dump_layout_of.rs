macro_rules! deps {
    () => {
        LayoutAbi!();
        LayoutOf!();
        UnwrapLayoutCx!();
        UnrecognizedArgument!();
        LayoutHomogeneousAggregate!();
        LayoutSize!();
        LayoutAlign!();
    };
}

macro_rules! dump_layout_of {
    () => {
        deps!();
        fn dump_layout_of (tcx : TyCtxt < '_ > , item_def_id : LocalDefId , attr : & Attribute) { let typing_env = ty :: TypingEnv :: post_analysis (tcx , item_def_id) ; let ty = tcx . type_of (item_def_id) . instantiate_identity () ; let span = tcx . def_span (item_def_id . to_def_id ()) ; if ! ensure_wf (tcx , typing_env , ty , item_def_id , span) { return ; } match tcx . layout_of (typing_env . as_query_input (ty)) { Ok (ty_layout) => { let meta_items = attr . meta_item_list () . unwrap_or_default () ; for meta_item in meta_items { match meta_item . name () { Some (sym :: abi) => { tcx . dcx () . emit_err (LayoutAbi { span , abi : format ! ("{:?}" , ty_layout . backend_repr) , }) ; } Some (sym :: align) => { tcx . dcx () . emit_err (LayoutAlign { span , align : format ! ("{:?}" , ty_layout . align) , }) ; } Some (sym :: size) => { tcx . dcx () . emit_err (LayoutSize { span , size : format ! ("{:?}" , ty_layout . size) }) ; } Some (sym :: homogeneous_aggregate) => { tcx . dcx () . emit_err (LayoutHomogeneousAggregate { span , homogeneous_aggregate : format ! ("{:?}" , ty_layout . homogeneous_aggregate (& UnwrapLayoutCx { tcx , typing_env })) , }) ; } Some (sym :: debug) => { let normalized_ty = tcx . normalize_erasing_regions (typing_env , ty) ; let ty_layout = format ! ("{:#?}" , * ty_layout) ; tcx . dcx () . emit_err (LayoutOf { span , normalized_ty , ty_layout }) ; } _ => { tcx . dcx () . emit_err (UnrecognizedArgument { span : meta_item . span () }) ; } } } } Err (layout_error) => { tcx . dcx () . emit_err (Spanned { node : layout_error . into_diagnostic () , span }) ; } } }
    };
}

dump_layout_of!();