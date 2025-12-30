// Generated macro for impl_33 (impl)
macro_rules! Depcrate_poolimpl_33 {
() => {
// Module: crate::pool
// Provides: {"impl_33"}
// Dependencies: {}
impl < T , C > std :: ops :: Deref for OwnedRef < T , C > where T : Clear + Default , C : cfg :: Config , { type Target = T ; fn deref (& self) -> & Self :: Target { self . value () } }
};
}
