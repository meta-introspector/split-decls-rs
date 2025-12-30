// Generated macro for impl_27 (impl)
macro_rules! Depcrate_poolimpl_27 {
() => {
// Module: crate::pool
// Provides: {"impl_27"}
// Dependencies: {}
impl < T , C : cfg :: Config > std :: ops :: Deref for RefMut < '_ , T , C > where T : Clear + Default , C : cfg :: Config , { type Target = T ; fn deref (& self) -> & Self :: Target { self . value () } }
};
}
