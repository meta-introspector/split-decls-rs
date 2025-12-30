// Generated macro for impl_22 (impl)
macro_rules! Depcrate_poolimpl_22 {
() => {
// Module: crate::pool
// Provides: {"impl_22"}
// Dependencies: {}
impl < T , C > std :: ops :: Deref for Ref < '_ , T , C > where T : Clear + Default , C : cfg :: Config , { type Target = T ; fn deref (& self) -> & Self :: Target { self . value () } }
};
}
