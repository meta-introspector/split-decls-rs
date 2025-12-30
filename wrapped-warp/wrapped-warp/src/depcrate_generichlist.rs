// Generated macro for HList (trait)
macro_rules! Depcrate_genericHList {
() => {
// Module: crate::generic
// Provides: {"HList"}
// Dependencies: {}
pub trait HList : Sized { type Tuple : Tuple < HList = Self > ; fn flatten (self) -> Self :: Tuple ; }
};
}
