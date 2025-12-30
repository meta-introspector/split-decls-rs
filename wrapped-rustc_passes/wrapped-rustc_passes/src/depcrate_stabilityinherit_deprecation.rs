// Generated macro for inherit_deprecation (function)
macro_rules! Depcrate_stabilityinherit_deprecation {
() => {
// Module: crate::stability
// Provides: {"inherit_deprecation"}
// Dependencies: {}
fn inherit_deprecation (def_kind : DefKind) -> bool { match def_kind { DefKind :: LifetimeParam | DefKind :: TyParam | DefKind :: ConstParam => false , _ => true , } }
};
}
