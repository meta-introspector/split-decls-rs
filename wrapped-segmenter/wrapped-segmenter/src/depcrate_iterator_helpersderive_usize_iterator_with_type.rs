// Generated macro for derive_usize_iterator_with_type (macro)
macro_rules! Depcrate_iterator_helpersderive_usize_iterator_with_type {
() => {
// Module: crate::iterator_helpers
// Provides: {"derive_usize_iterator_with_type"}
// Dependencies: {}
macro_rules ! derive_usize_iterator_with_type { ($ ty : tt , $ ($ lt : lifetime) ,*) => { impl <$ ($ lt ,) * 's , Y : RuleBreakType > Iterator for $ ty <$ ($ lt ,) * 's , Y > { type Item = usize ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () } } } ; }
};
}
