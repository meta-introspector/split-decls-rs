// Generated macro for impl_226 (impl)
macro_rules! Depcrate_search_graphimpl_226 {
() => {
// Module: crate::search_graph
// Provides: {"impl_226"}
// Dependencies: {}
impl PathKind { # [doc = " Returns the path kind when merging `self` with `rest`."] # [doc = ""] # [doc = " Given an inductive path `self` and a coinductive path `rest`,"] # [doc = " the path `self -> rest` would be coinductive."] # [doc = ""] # [doc = " This operation represents an ordering and would be equivalent"] # [doc = " to `max(self, rest)`."] fn extend (self , rest : PathKind) -> PathKind { match (self , rest) { (PathKind :: ForcedAmbiguity , _) | (_ , PathKind :: ForcedAmbiguity) => { PathKind :: ForcedAmbiguity } (PathKind :: Coinductive , _) | (_ , PathKind :: Coinductive) => PathKind :: Coinductive , (PathKind :: Unknown , _) | (_ , PathKind :: Unknown) => PathKind :: Unknown , (PathKind :: Inductive , PathKind :: Inductive) => PathKind :: Inductive , } } }
};
}
