// Generated macro for visit_mentioned_item (function)
macro_rules! Depcrate_collectorvisit_mentioned_item {
() => {
// Module: crate::collector
// Provides: {"visit_mentioned_item"}
// Dependencies: {}
# [doc = " `item` must be already monomorphized."] # [instrument (skip (tcx , span , output) , level = "debug")] fn visit_mentioned_item < 'tcx > (tcx : TyCtxt < 'tcx > , item : & MentionedItem < 'tcx > , span : Span , output : & mut MonoItems < 'tcx > ,) { match * item { MentionedItem :: Fn (ty) => { if let ty :: FnDef (def_id , args) = * ty . kind () { let instance = Instance :: expect_resolve (tcx , ty :: TypingEnv :: fully_monomorphized () , def_id , args , span ,) ; visit_instance_use (tcx , instance , true , span , output) ; } } MentionedItem :: Drop (ty) => { visit_drop_use (tcx , ty , true , span , output) ; } MentionedItem :: UnsizeCast { source_ty , target_ty } => { let (source_ty , target_ty) = find_tails_for_unsizing (tcx . at (span) , source_ty , target_ty) ; if target_ty . is_trait () && ! source_ty . is_trait () { create_mono_items_for_vtable_methods (tcx , target_ty , source_ty , span , output) ; } } MentionedItem :: Closure (source_ty) => { if let ty :: Closure (def_id , args) = * source_ty . kind () { let instance = Instance :: resolve_closure (tcx , def_id , args , ty :: ClosureKind :: FnOnce) ; if tcx . should_codegen_locally (instance) { output . push (create_fn_mono_item (tcx , instance , span)) ; } } else { bug ! () } } } }
};
}
