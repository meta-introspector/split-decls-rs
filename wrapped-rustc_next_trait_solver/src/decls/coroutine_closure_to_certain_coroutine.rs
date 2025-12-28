macro_rules! coroutine_closure_to_certain_coroutine {
    () => {
        # [doc = " Given a coroutine-closure, project to its returned coroutine when we are *certain*"] # [doc = " that the closure's kind is compatible with the goal."] fn coroutine_closure_to_certain_coroutine < I : Interner > (cx : I , goal_kind : ty :: ClosureKind , goal_region : I :: Region , def_id : I :: CoroutineClosureId , args : ty :: CoroutineClosureArgs < I > , sig : ty :: CoroutineClosureSignature < I > ,) -> I :: Ty { sig . to_coroutine_given_kind_and_upvars (cx , args . parent_args () , cx . coroutine_for_closure (def_id) , goal_kind , goal_region , args . tupled_upvars_ty () , args . coroutine_captures_by_ref_ty () ,) }
    };
}

coroutine_closure_to_certain_coroutine!();