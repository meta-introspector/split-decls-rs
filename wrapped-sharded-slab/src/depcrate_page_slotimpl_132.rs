// Generated macro for impl_132 (impl)
macro_rules! Depcrate_page_slotimpl_132 {
() => {
// Module: crate::page::slot
// Provides: {"impl_132"}
// Dependencies: {}
impl < C : cfg :: Config > Pack < C > for LifecycleGen < C > { const LEN : usize = Generation :: < C > :: LEN ; type Prev = RefCount < C > ; fn from_usize (value : usize) -> Self { Self (Generation :: from_usize (value)) } fn as_usize (& self) -> usize { self . 0 . as_usize () } }
};
}
