// Generated macro for impl_230 (impl)
macro_rules! Depcrate_impls_initializedimpl_230 {
() => {
// Module: crate::impls::initialized
// Provides: {"impl_230"}
// Dependencies: {}
impl < 'tcx > MaybePlacesSwitchIntData < 'tcx > { fn new (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , block : mir :: BasicBlock , discr : & mir :: Operand < 'tcx > ,) -> Option < Self > { let Some (discr) = discr . place () else { return None } ; let block_data = & body [block] ; for statement in block_data . statements . iter () . rev () { match statement . kind { mir :: StatementKind :: Assign (box (lhs , mir :: Rvalue :: Discriminant (enum_place))) if lhs == discr => { match enum_place . ty (body , tcx) . ty . kind () { ty :: Adt (enum_def , _) => { return Some (MaybePlacesSwitchIntData { enum_place , discriminants : enum_def . discriminants (tcx) . collect () , index : 0 , }) ; } ty :: Coroutine (..) => break , t => bug ! ("`discriminant` called on unexpected type {:?}" , t) , } } mir :: StatementKind :: Coverage (_) => continue , _ => break , } } None } }
};
}
