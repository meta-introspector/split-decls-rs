// Generated macro for Relate (trait)
macro_rules! Depcrate_relateRelate {
() => {
// Module: crate::relate
// Provides: {"Relate"}
// Dependencies: {}
pub trait Relate < I : Interner > : TypeFoldable < I > + PartialEq + Copy { fn relate < R : TypeRelation < I > > (relation : & mut R , a : Self , b : Self) -> RelateResult < I , Self > ; }
};
}
