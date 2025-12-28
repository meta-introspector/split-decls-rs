macro_rules! deps {
    () => {
        WantedConstant!();
    };
}

macro_rules! UnreachablePattern {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (mir_build_unreachable_pattern)] pub (crate) struct UnreachablePattern < 'tcx > { # [label] pub (crate) span : Option < Span > , # [label (mir_build_unreachable_matches_no_values)] pub (crate) matches_no_values : Option < Span > , pub (crate) matches_no_values_ty : Ty < 'tcx > , # [note (mir_build_unreachable_uninhabited_note)] pub (crate) uninhabited_note : Option < () > , # [label (mir_build_unreachable_covered_by_catchall)] pub (crate) covered_by_catchall : Option < Span > , # [subdiagnostic] pub (crate) wanted_constant : Option < WantedConstant > , # [note (mir_build_unreachable_pattern_const_reexport_accessible)] pub (crate) accessible_constant : Option < Span > , # [note (mir_build_unreachable_pattern_const_inaccessible)] pub (crate) inaccessible_constant : Option < Span > , # [note (mir_build_unreachable_pattern_let_binding)] pub (crate) pattern_let_binding : Option < Span > , # [label (mir_build_unreachable_covered_by_one)] pub (crate) covered_by_one : Option < Span > , # [note (mir_build_unreachable_covered_by_many)] pub (crate) covered_by_many : Option < MultiSpan > , pub (crate) covered_by_many_n_more_count : usize , # [suggestion (code = "" , applicability = "machine-applicable")] pub (crate) suggest_remove : Option < Span > , }
    };
}

UnreachablePattern!()