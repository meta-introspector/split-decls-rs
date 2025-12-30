// Generated macro for inherit_stability (function)
macro_rules! Depcrate_stabilityinherit_stability {
() => {
// Module: crate::stability
// Provides: {"inherit_stability"}
// Dependencies: {}
fn inherit_stability (def_kind : DefKind) -> bool { match def_kind { DefKind :: Field | DefKind :: Variant | DefKind :: Ctor (..) => true , _ => false , } }
};
}
