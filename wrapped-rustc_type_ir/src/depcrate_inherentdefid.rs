// Generated macro for DefId (trait)
macro_rules! Depcrate_inherentDefId {
() => {
// Module: crate::inherent
// Provides: {"DefId"}
// Dependencies: {}
pub trait DefId < I : Interner > : Copy + Debug + Hash + Eq + TypeFoldable < I > { fn is_local (self) -> bool ; fn as_local (self) -> Option < I :: LocalDefId > ; }
};
}
