// Generated macro for impl_464 (impl)
macro_rules! Depcrate_storageimpl_464 {
() => {
// Module: crate::storage
// Provides: {"impl_464"}
// Dependencies: {}
impl < Db : Database > Clone for Storage < Db > { fn clone (& self) -> Self { Self { handle : self . handle . clone () , zalsa_local : ZalsaLocal :: new () , } } }
};
}
