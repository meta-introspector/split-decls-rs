// Generated macro for impl_73 (impl)
macro_rules! Depcrate_clearimpl_73 {
() => {
// Module: crate::clear
// Provides: {"impl_73"}
// Dependencies: {}
impl < T , S > Clear for collections :: HashSet < T , S > where T : hash :: Hash + Eq , S : hash :: BuildHasher , { # [inline] fn clear (& mut self) { collections :: HashSet :: clear (self) } }
};
}
