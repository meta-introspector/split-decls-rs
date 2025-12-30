// Generated macro for UpcastFrom (trait)
macro_rules! Depcrate_upcastUpcastFrom {
() => {
// Module: crate::upcast
// Provides: {"UpcastFrom"}
// Dependencies: {}
# [doc = " A `From`-like trait that takes `TyCtxt` to perform interner-specific transformations."] pub trait UpcastFrom < I , T > { fn upcast_from (from : T , interner : I) -> Self ; }
};
}
