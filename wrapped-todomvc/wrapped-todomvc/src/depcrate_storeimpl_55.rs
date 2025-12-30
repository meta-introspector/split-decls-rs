// Generated macro for impl_55 (impl)
macro_rules! Depcrate_storeimpl_55 {
() => {
// Module: crate::store
// Provides: {"impl_55"}
// Dependencies: {}
impl ItemUpdate { fn id (& self) -> String { match self { ItemUpdate :: Title { id , .. } => id . clone () , ItemUpdate :: Completed { id , .. } => id . clone () , } } }
};
}
