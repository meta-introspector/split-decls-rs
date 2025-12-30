// Generated macro for impl_93 (impl)
macro_rules! Depcrate_check_exportimpl_93 {
() => {
// Module: crate::check_export
// Provides: {"impl_93"}
// Dependencies: {}
impl < 'tcx , 'a > TypeVisitor < TyCtxt < 'tcx > > for ExportableItemsChecker < 'tcx , 'a > { type Result = ControlFlow < Ty < 'tcx > > ; fn visit_ty (& mut self , ty : Ty < 'tcx >) -> Self :: Result { match ty . kind () { ty :: Adt (adt_def , _) => { let did = adt_def . did () ; let exportable = if did . is_local () { self . exportable_items . contains (& did) } else { self . tcx . is_exportable (did) } ; if ! exportable { return ControlFlow :: Break (ty) ; } for variant in adt_def . variants () { for field in & variant . fields { let field_ty = self . tcx . type_of (field . did) . instantiate_identity () ; field_ty . visit_with (self) ? ; } } return ty . super_visit_with (self) ; } ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Bool | ty :: Char | ty :: Error (_) => { } ty :: Array (_ , _) | ty :: Ref (_ , _ , _) | ty :: Param (_) | ty :: Closure (_ , _) | ty :: Dynamic (_ , _ , _) | ty :: Coroutine (_ , _) | ty :: Foreign (_) | ty :: Str | ty :: Tuple (_) | ty :: Pat (..) | ty :: Slice (_) | ty :: RawPtr (_ , _) | ty :: FnDef (_ , _) | ty :: FnPtr (_ , _) | ty :: CoroutineClosure (_ , _) | ty :: CoroutineWitness (_ , _) | ty :: Never | ty :: UnsafeBinder (_) | ty :: Alias (ty :: AliasTyKind :: Opaque , _) => { return ControlFlow :: Break (ty) ; } ty :: Alias (..) | ty :: Infer (_) | ty :: Placeholder (_) | ty :: Bound (..) => unreachable ! () , } ControlFlow :: Continue (()) } }
};
}
