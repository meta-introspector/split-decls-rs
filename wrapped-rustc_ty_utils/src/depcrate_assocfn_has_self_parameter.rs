// Generated macro for fn_has_self_parameter (function)
macro_rules! Depcrate_assocfn_has_self_parameter {
() => {
// Module: crate::assoc
// Provides: {"fn_has_self_parameter"}
// Dependencies: {}
fn fn_has_self_parameter (tcx : TyCtxt < '_ > , owner_id : hir :: OwnerId) -> bool { matches ! (tcx . fn_arg_idents (owner_id . def_id) , [Some (Ident { name : kw :: SelfLower , .. }) , ..]) }
};
}
