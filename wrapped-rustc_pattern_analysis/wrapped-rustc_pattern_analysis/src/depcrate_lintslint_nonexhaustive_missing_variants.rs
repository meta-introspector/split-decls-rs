// Generated macro for lint_nonexhaustive_missing_variants (function)
macro_rules! Depcrate_lintslint_nonexhaustive_missing_variants {
() => {
// Module: crate::lints
// Provides: {"lint_nonexhaustive_missing_variants"}
// Dependencies: {}
pub (crate) fn lint_nonexhaustive_missing_variants < 'p , 'tcx > (rcx : & RustcPatCtxt < 'p , 'tcx > , arms : & [MatchArm < 'p , RustcPatCtxt < 'p , 'tcx > >] , pat_column : & PatternColumn < 'p , RustcPatCtxt < 'p , 'tcx > > , scrut_ty : RevealedTy < 'tcx > ,) -> Result < () , ErrorGuaranteed > { if ! matches ! (rcx . tcx . lint_level_at_node (NON_EXHAUSTIVE_OMITTED_PATTERNS , rcx . match_lint_level) . level , rustc_session :: lint :: Level :: Allow) { let witnesses = collect_nonexhaustive_missing_variants (rcx , pat_column) ? ; if ! witnesses . is_empty () { rcx . tcx . emit_node_span_lint (NON_EXHAUSTIVE_OMITTED_PATTERNS , rcx . match_lint_level , rcx . scrut_span , NonExhaustiveOmittedPattern { scrut_ty : scrut_ty . inner () , uncovered : Uncovered :: new (rcx . scrut_span , rcx , witnesses) , } ,) ; } } else { for arm in arms { let LevelAndSource { level , src , .. } = rcx . tcx . lint_level_at_node (NON_EXHAUSTIVE_OMITTED_PATTERNS , arm . arm_data) ; if ! matches ! (level , rustc_session :: lint :: Level :: Allow) { let decorator = NonExhaustiveOmittedPatternLintOnArm { lint_span : src . span () , suggest_lint_on_match : rcx . whole_match_span . map (| span | span . shrink_to_lo ()) , lint_level : level . as_str () , lint_name : "non_exhaustive_omitted_patterns" , } ; use rustc_errors :: LintDiagnostic ; let mut err = rcx . tcx . dcx () . struct_span_warn (arm . pat . data () . span , "") ; decorator . decorate_lint (& mut err) ; err . emit () ; } } } Ok (()) }
};
}
