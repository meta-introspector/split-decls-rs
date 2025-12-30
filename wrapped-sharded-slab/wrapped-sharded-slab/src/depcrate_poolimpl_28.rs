// Generated macro for impl_28 (impl)
macro_rules! Depcrate_poolimpl_28 {
() => {
// Module: crate::pool
// Provides: {"impl_28"}
// Dependencies: {}
impl < T , C > std :: ops :: DerefMut for RefMut < '_ , T , C > where T : Clear + Default , C : cfg :: Config , { fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { self . inner . value_mut () } } }
};
}
