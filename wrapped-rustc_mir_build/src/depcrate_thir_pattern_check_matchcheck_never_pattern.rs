// Generated macro for check_never_pattern (function)
macro_rules! Depcrate_thir_pattern_check_matchcheck_never_pattern {
() => {
// Module: crate::thir::pattern::check_match
// Provides: {"check_never_pattern"}
// Dependencies: {}
# [doc = " Check that never patterns are only used on inhabited types."] fn check_never_pattern < 'tcx > (cx : & PatCtxt < '_ , 'tcx > , pat : & Pat < 'tcx > ,) -> Result < () , ErrorGuaranteed > { if let PatKind :: Never = pat . kind { if ! cx . is_uninhabited (pat . ty) { return Err (cx . tcx . dcx () . emit_err (NonEmptyNeverPattern { span : pat . span , ty : pat . ty })) ; } } Ok (()) }
};
}
