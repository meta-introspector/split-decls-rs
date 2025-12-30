// Generated macro for impl_40 (impl)
macro_rules! Depcrate_poolimpl_40 {
() => {
// Module: crate::pool
// Provides: {"impl_40"}
// Dependencies: {}
impl < T , C > std :: ops :: Deref for OwnedRefMut < T , C > where T : Clear + Default , C : cfg :: Config , { type Target = T ; fn deref (& self) -> & Self :: Target { self . value () } }
};
}
