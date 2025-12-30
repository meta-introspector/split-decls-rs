// Generated macro for impl_452 (impl)
macro_rules! Depcrate_storageimpl_452 {
() => {
// Module: crate::storage
// Provides: {"impl_452"}
// Dependencies: {}
impl < Db > Clone for StorageHandle < Db > { fn clone (& self) -> Self { Self { zalsa_impl : self . zalsa_impl . clone () , coordinate : CoordinateDrop (Arc :: clone (& self . coordinate)) , phantom : PhantomData , } } }
};
}
