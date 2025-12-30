// Generated macro for impl_144 (impl)
macro_rules! Depcrate_tendrilimpl_144 {
() => {
// Module: crate::tendril
// Provides: {"impl_144"}
// Dependencies: {}
impl < F , A > hash :: Hash for Tendril < F , A > where F : fmt :: Format , A : Atomicity , { # [inline] fn hash < H : hash :: Hasher > (& self , hasher : & mut H) { self . as_byte_slice () . hash (hasher) } }
};
}
