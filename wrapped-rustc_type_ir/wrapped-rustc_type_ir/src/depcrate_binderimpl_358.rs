// Generated macro for impl_358 (impl)
macro_rules! Depcrate_binderimpl_358 {
() => {
// Module: crate::binder
// Provides: {"impl_358"}
// Dependencies: {}
impl < Iter : IntoIterator > Iterator for IterIdentityCopied < Iter > where Iter :: Item : Deref , < Iter :: Item as Deref > :: Target : Copy , { type Item = < Iter :: Item as Deref > :: Target ; fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| i | * i) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
