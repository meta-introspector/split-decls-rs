macro_rules! deps {
    () => {
        Builder!();
        PlaceBuilder!();
    };
}

macro_rules! to_upvars_resolved_place_builder {
    () => {
        deps!();
        # [doc = " Takes an upvar place and tries to resolve it into a `PlaceBuilder`"] # [doc = " with `PlaceBase::Local`"] # [instrument (level = "trace" , skip (cx) , ret)] fn to_upvars_resolved_place_builder < 'tcx > (cx : & Builder < '_ , 'tcx > , var_hir_id : LocalVarId , closure_def_id : LocalDefId , projection : & [PlaceElem < 'tcx >] ,) -> Option < PlaceBuilder < 'tcx > > { let Some ((capture_index , capture)) = find_capture_matching_projections (& cx . upvars , var_hir_id , projection) else { let closure_span = cx . tcx . def_span (closure_def_id) ; if ! enable_precise_capture (closure_span) { bug ! ("No associated capture found for {:?}[{:#?}] even though \
                    capture_disjoint_fields isn't enabled" , var_hir_id , projection) } else { debug ! ("No associated capture found for {:?}[{:#?}]" , var_hir_id , projection ,) ; } return None ; } ; let capture_info = & cx . upvars [capture_index] ; let mut upvar_resolved_place_builder = PlaceBuilder :: from (capture_info . use_place) ; trace ! (? capture . captured_place , ? projection) ; let remaining_projections = strip_prefix (capture . captured_place . place . base_ty , projection , & capture . captured_place . place . projections ,) ; upvar_resolved_place_builder . projection . extend (remaining_projections) ; Some (upvar_resolved_place_builder) }
    };
}

to_upvars_resolved_place_builder!()