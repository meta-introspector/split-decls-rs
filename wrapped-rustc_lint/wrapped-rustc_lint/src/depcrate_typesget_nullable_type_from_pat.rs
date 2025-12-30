// Generated macro for get_nullable_type_from_pat (function)
macro_rules! Depcrate_typesget_nullable_type_from_pat {
() => {
// Module: crate::types
// Provides: {"get_nullable_type_from_pat"}
// Dependencies: {}
fn get_nullable_type_from_pat < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , base : Ty < 'tcx > , pat : ty :: Pattern < 'tcx > ,) -> Option < Ty < 'tcx > > { match * pat { ty :: PatternKind :: Range { .. } => get_nullable_type (tcx , typing_env , base) , ty :: PatternKind :: Or (patterns) => { let first = get_nullable_type_from_pat (tcx , typing_env , base , patterns [0]) ? ; for & pat in & patterns [1 ..] { assert_eq ! (first , get_nullable_type_from_pat (tcx , typing_env , base , pat) ?) ; } Some (first) } } }
};
}
