// Generated macro for dropee_emit_retag (function)
macro_rules! Depcrate_shimdropee_emit_retag {
() => {
// Module: crate::shim
// Provides: {"dropee_emit_retag"}
// Dependencies: {}
fn dropee_emit_retag < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx > , mut dropee_ptr : Place < 'tcx > , span : Span ,) -> Place < 'tcx > { if tcx . sess . opts . unstable_opts . mir_emit_retag { let source_info = SourceInfo :: outermost (span) ; let reborrow = Rvalue :: Ref (tcx . lifetimes . re_erased , BorrowKind :: Mut { kind : MutBorrowKind :: Default } , tcx . mk_place_deref (dropee_ptr) ,) ; let ref_ty = reborrow . ty (body . local_decls () , tcx) ; dropee_ptr = body . local_decls . push (LocalDecl :: new (ref_ty , span)) . into () ; let new_statements = [StatementKind :: Assign (Box :: new ((dropee_ptr , reborrow))) , StatementKind :: Retag (RetagKind :: FnEntry , Box :: new (dropee_ptr)) ,] ; for s in new_statements { body . basic_blocks_mut () [START_BLOCK] . statements . push (Statement :: new (source_info , s)) ; } } dropee_ptr }
};
}
