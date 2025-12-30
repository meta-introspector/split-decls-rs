// Generated macro for PlaceholderLike (trait)
macro_rules! Depcrate_inherentPlaceholderLike {
() => {
// Module: crate::inherent
// Provides: {"PlaceholderLike"}
// Dependencies: {}
# [doc = " Common capabilities of placeholder kinds"] pub trait PlaceholderLike < I : Interner > : Copy + Debug + Hash + Eq { fn universe (self) -> ty :: UniverseIndex ; fn var (self) -> ty :: BoundVar ; type Bound : BoundVarLike < I > ; fn new (ui : ty :: UniverseIndex , bound : Self :: Bound) -> Self ; fn new_anon (ui : ty :: UniverseIndex , var : ty :: BoundVar) -> Self ; fn with_updated_universe (self , ui : ty :: UniverseIndex) -> Self ; }
};
}
