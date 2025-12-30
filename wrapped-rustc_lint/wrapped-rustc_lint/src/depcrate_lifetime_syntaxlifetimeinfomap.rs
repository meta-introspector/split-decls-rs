// Generated macro for LifetimeInfoMap (type)
macro_rules! Depcrate_lifetime_syntaxLifetimeInfoMap {
() => {
// Module: crate::lifetime_syntax
// Provides: {"LifetimeInfoMap"}
// Dependencies: {}
type LifetimeInfoMap < 'tcx > = FxIndexMap < & 'tcx hir :: LifetimeKind , Vec < Info < 'tcx > > > ;
};
}
