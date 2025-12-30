// Generated macro for impl_18 (impl)
macro_rules! Depcrate_deimpl_18 {
() => {
// Module: crate::de
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'de > Iterator for PartIterator < 'de > { type Item = (Part < 'de > , Part < 'de >) ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () . map (| (k , v) | (Part (k) , Part (v))) } }
};
}
