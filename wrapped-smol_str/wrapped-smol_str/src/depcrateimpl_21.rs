// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl hash :: Hash for SmolStr { fn hash < H : hash :: Hasher > (& self , hasher : & mut H) { self . as_str () . hash (hasher) ; } }
};
}
