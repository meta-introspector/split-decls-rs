// Generated macro for impl_62 (impl)
macro_rules! Depcrate_framework_cursorimpl_62 {
() => {
// Module: crate::framework::cursor
// Provides: {"impl_62"}
// Dependencies: {}
impl < T > DerefMut for CowMut < '_ , T > { fn deref_mut (& mut self) -> & mut T { match self { CowMut :: BorrowedMut (borrowed) => borrowed , CowMut :: Owned (owned) => owned , } } }
};
}
