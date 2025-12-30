// Generated macro for impl_41 (impl)
macro_rules! Depcrate_poolimpl_41 {
() => {
// Module: crate::pool
// Provides: {"impl_41"}
// Dependencies: {}
impl < T , C > std :: ops :: DerefMut for OwnedRefMut < T , C > where T : Clear + Default , C : cfg :: Config , { fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { self . inner . value_mut () } } }
};
}
