// Generated macro for impl_7 (impl)
macro_rules! Depcrate_cacheimpl_7 {
() => {
// Module: crate::cache
// Provides: {"impl_7"}
// Dependencies: {}
impl < Key : Clone , Value : Clone > Clone for Cache < Key , Value > { fn clone (& self) -> Self { Self { hashmap : Lock :: new (self . hashmap . borrow () . clone ()) } } }
};
}
