// Generated macro for impl_72 (impl)
macro_rules! Depcrate_clearimpl_72 {
() => {
// Module: crate::clear
// Provides: {"impl_72"}
// Dependencies: {}
impl < K , V , S > Clear for collections :: HashMap < K , V , S > where K : hash :: Hash + Eq , S : hash :: BuildHasher , { # [inline] fn clear (& mut self) { collections :: HashMap :: clear (self) } }
};
}
