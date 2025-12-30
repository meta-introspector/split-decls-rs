// Generated macro for impl_237 (impl)
macro_rules! Depcrate_search_graphimpl_237 {
() => {
// Module: crate::search_graph
// Provides: {"impl_237"}
// Dependencies: {}
impl From < PathKind > for PathsToNested { fn from (path : PathKind) -> PathsToNested { match path { PathKind :: Inductive => PathsToNested :: INDUCTIVE , PathKind :: Unknown => PathsToNested :: UNKNOWN , PathKind :: Coinductive => PathsToNested :: COINDUCTIVE , PathKind :: ForcedAmbiguity => PathsToNested :: FORCED_AMBIGUITY , } } }
};
}
