// Generated macro for coroutine_closure_to_ambiguous_coroutine (function)
macro_rules! Depcrate_solve_assembly_structural_traitscoroutine_closure_to_ambiguous_coroutine {
() => {
// Module: crate::solve::assembly::structural_traits
// Provides: {"coroutine_closure_to_ambiguous_coroutine"}
// Dependencies: {}
# [doc = " Given a coroutine-closure, project to its returned coroutine when we are *not certain*"] # [doc = " that the closure's kind is compatible with the goal, and therefore also don't know"] # [doc = " yet what the closure's upvars are."] # [doc = ""] # [doc = " Note that we do not also push a `AsyncFnKindHelper` goal here."] fn coroutine_closure_to_ambiguous_coroutine < I : Interner > (cx : I , goal_kind : ty :: ClosureKind , goal_region : I :: Region , def_id : I :: CoroutineClosureId , args : ty :: CoroutineClosureArgs < I > , sig : ty :: CoroutineClosureSignature < I > ,) -> I :: Ty { let upvars_projection_def_id = cx . require_lang_item (SolverLangItem :: AsyncFnKindUpvars) ; let tupled_upvars_ty = Ty :: new_projection (cx , upvars_projection_def_id , [I :: GenericArg :: from (args . kind_ty ()) , Ty :: from_closure_kind (cx , goal_kind) . into () , goal_region . into () , sig . tupled_inputs_ty . into () , args . tupled_upvars_ty () . into () , args . coroutine_captures_by_ref_ty () . into () ,] ,) ; sig . to_coroutine (cx , args . parent_args () , Ty :: from_closure_kind (cx , goal_kind) , cx . coroutine_for_closure (def_id) , tupled_upvars_ty ,) }
};
}
