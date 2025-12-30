// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl < S > Drop for Guard < '_ , S > where AllowStd < S > : Read + Write , { fn drop (& mut self) { (self . 0) . 0 . get_mut () . context = null_mut () ; } }
};
}
