// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
impl < Z > Zeroize for IterMut < '_ , Z > where Z : Zeroize , { fn zeroize (& mut self) { for elem in self { elem . zeroize () ; } } }
};
}
