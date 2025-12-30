// Generated macro for UnDerefer (struct)
macro_rules! Depcrate_un_dereferUnDerefer {
() => {
// Module: crate::un_derefer
// Provides: {"UnDerefer"}
// Dependencies: {}
# [doc = " Used for reverting changes made by `DerefSeparator`"] # [derive (Default , Debug)] pub (crate) struct UnDerefer < 'tcx > { deref_chains : FxHashMap < Local , Vec < PlaceRef < 'tcx > > > , }
};
}
