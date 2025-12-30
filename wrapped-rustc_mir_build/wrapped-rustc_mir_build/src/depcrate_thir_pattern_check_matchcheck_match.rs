// Generated macro for check_match (function)
macro_rules! Depcrate_thir_pattern_check_matchcheck_match {
() => {
// Module: crate::thir::pattern::check_match
// Provides: {"check_match"}
// Dependencies: {}
pub (crate) fn check_match (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Result < () , ErrorGuaranteed > { let typeck_results = tcx . typeck (def_id) ; let (thir , expr) = tcx . thir_body (def_id) ? ; let thir = thir . borrow () ; let pattern_arena = TypedArena :: default () ; let dropless_arena = DroplessArena :: default () ; let mut visitor = MatchVisitor { tcx , thir : & * thir , typeck_results , typing_env : ty :: TypingEnv :: non_body_analysis (tcx , def_id) , lint_level : tcx . local_def_id_to_hir_id (def_id) , let_source : LetSource :: None , pattern_arena : & pattern_arena , dropless_arena : & dropless_arena , error : Ok (()) , } ; visitor . visit_expr (& thir [expr]) ; let origin = match tcx . def_kind (def_id) { DefKind :: AssocFn | DefKind :: Fn => "function argument" , DefKind :: Closure => "closure argument" , _ if thir . params . is_empty () => "" , kind => bug ! ("unexpected function parameters in THIR: {kind:?} {def_id:?}") , } ; for param in thir . params . iter () { if let Some (box ref pattern) = param . pat { visitor . check_binding_is_irrefutable (pattern , origin , None , None) ; } } visitor . error }
};
}
