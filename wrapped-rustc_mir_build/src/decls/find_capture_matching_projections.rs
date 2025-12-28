macro_rules! deps {
    () => {
        Capture!();
        CaptureMap!();
    };
}

macro_rules! find_capture_matching_projections {
    () => {
        deps!();
        # [doc = " Given a closure, returns the index of a capture within the desugared closure struct and the"] # [doc = " `ty::CapturedPlace` which is the ancestor of the Place represented using the `var_hir_id`"] # [doc = " and `projection`."] # [doc = ""] # [doc = " Note there will be at most one ancestor for any given Place."] # [doc = ""] # [doc = " Returns None, when the ancestor is not found."] fn find_capture_matching_projections < 'a , 'tcx > (upvars : & 'a CaptureMap < 'tcx > , var_hir_id : LocalVarId , projections : & [PlaceElem < 'tcx >] ,) -> Option < (usize , & 'a Capture < 'tcx >) > { let hir_projections = convert_to_hir_projections_and_truncate_for_capture (projections) ; upvars . get_by_key_enumerated (var_hir_id . 0 . local_id) . find (| (_ , capture) | { let possible_ancestor_proj_kinds : Vec < _ > = capture . captured_place . place . projections . iter () . map (| proj | proj . kind) . collect () ; is_ancestor_or_same_capture (& possible_ancestor_proj_kinds , & hir_projections) }) }
    };
}

find_capture_matching_projections!()