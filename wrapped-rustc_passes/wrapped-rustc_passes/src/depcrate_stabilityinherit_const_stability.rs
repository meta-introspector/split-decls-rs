// Generated macro for inherit_const_stability (function)
macro_rules! Depcrate_stabilityinherit_const_stability {
() => {
// Module: crate::stability
// Provides: {"inherit_const_stability"}
// Dependencies: {}
fn inherit_const_stability (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { let def_kind = tcx . def_kind (def_id) ; match def_kind { DefKind :: AssocFn | DefKind :: AssocTy | DefKind :: AssocConst => { match tcx . def_kind (tcx . local_parent (def_id)) { DefKind :: Impl { of_trait : true } => true , _ => false , } } _ => false , } }
};
}
