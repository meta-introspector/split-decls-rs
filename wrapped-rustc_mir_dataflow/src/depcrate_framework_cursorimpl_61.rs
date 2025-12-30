// Generated macro for impl_61 (impl)
macro_rules! Depcrate_framework_cursorimpl_61 {
() => {
// Module: crate::framework::cursor
// Provides: {"impl_61"}
// Dependencies: {}
impl < T > Deref for CowMut < '_ , T > { type Target = T ; fn deref (& self) -> & T { match self { CowMut :: BorrowedMut (borrowed) => borrowed , CowMut :: Owned (owned) => owned , } } }
};
}
