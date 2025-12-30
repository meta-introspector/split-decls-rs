// Generated macro for params_in_repr_ty (function)
macro_rules! Depcrate_representabilityparams_in_repr_ty {
() => {
// Module: crate::representability
// Provides: {"params_in_repr_ty"}
// Dependencies: {}
fn params_in_repr_ty < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , params_in_repr : & mut DenseBitSet < u32 >) { match * ty . kind () { ty :: Adt (adt , args) => { let inner_params_in_repr = tcx . params_in_repr (adt . did ()) ; for (i , arg) in args . iter () . enumerate () { if let ty :: GenericArgKind :: Type (ty) = arg . kind () { if inner_params_in_repr . contains (i as u32) { params_in_repr_ty (tcx , ty , params_in_repr) ; } } } } ty :: Array (ty , _) => params_in_repr_ty (tcx , ty , params_in_repr) , ty :: Tuple (tys) => tys . iter () . for_each (| ty | params_in_repr_ty (tcx , ty , params_in_repr)) , ty :: Param (param) => { params_in_repr . insert (param . index) ; } _ => { } } }
};
}
