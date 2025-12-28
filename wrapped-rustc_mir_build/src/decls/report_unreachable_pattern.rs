macro_rules! deps {
    () => {
        PatCtxt!();
        UnreachablePattern!();
    };
}

macro_rules! report_unreachable_pattern {
    () => {
        deps!();
        # [doc = " Report unreachable arms, if any."] fn report_unreachable_pattern < 'p , 'tcx > (cx : & PatCtxt < 'p , 'tcx > , hir_id : HirId , pat : & DeconstructedPat < 'p , 'tcx > , explanation : & RedundancyExplanation < 'p , 'tcx > , whole_arm_span : Option < Span > ,) { static CAP_COVERED_BY_MANY : usize = 4 ; let pat_span = pat . data () . span ; let mut lint = UnreachablePattern { span : Some (pat_span) , matches_no_values : None , matches_no_values_ty : * * pat . ty () , uninhabited_note : None , covered_by_catchall : None , covered_by_one : None , covered_by_many : None , covered_by_many_n_more_count : 0 , wanted_constant : None , accessible_constant : None , inaccessible_constant : None , pattern_let_binding : None , suggest_remove : None , } ; match explanation . covered_by . as_slice () { [] => { lint . span = None ; lint . uninhabited_note = Some (()) ; lint . matches_no_values = Some (pat_span) ; lint . suggest_remove = whole_arm_span ; pat . walk (& mut | subpat | { let ty = * * subpat . ty () ; if cx . is_uninhabited (ty) { lint . matches_no_values_ty = ty ; false } else if matches ! (subpat . ctor () , Constructor :: Ref | Constructor :: UnionField) { false } else { true } }) ; } [covering_pat] if pat_is_catchall (covering_pat) => { let pat = covering_pat . data () ; lint . covered_by_catchall = Some (pat . span) ; find_fallback_pattern_typo (cx , hir_id , pat , & mut lint) ; } [covering_pat] => { lint . covered_by_one = Some (covering_pat . data () . span) ; } covering_pats => { let mut iter = covering_pats . iter () ; let mut multispan = MultiSpan :: from_span (pat_span) ; for p in iter . by_ref () . take (CAP_COVERED_BY_MANY) { multispan . push_span_label (p . data () . span , fluent :: mir_build_unreachable_matches_same_values ,) ; } let remain = iter . count () ; if remain == 0 { multispan . push_span_label (pat_span , fluent :: mir_build_unreachable_making_this_unreachable ,) ; } else { lint . covered_by_many_n_more_count = remain ; multispan . push_span_label (pat_span , fluent :: mir_build_unreachable_making_this_unreachable_n_more ,) ; } lint . covered_by_many = Some (multispan) ; } } cx . tcx . emit_node_span_lint (UNREACHABLE_PATTERNS , hir_id , pat_span , lint) ; }
    };
}

report_unreachable_pattern!()