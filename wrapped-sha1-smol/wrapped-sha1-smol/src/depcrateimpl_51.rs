// Generated macro for impl_51 (impl)
macro_rules! Depcrateimpl_51 {
() => {
// Module: crate
// Provides: {"impl_51"}
// Dependencies: {}
impl hash :: Hash for Blocks { fn hash < H : hash :: Hasher > (& self , state : & mut H) { self . len . hash (state) ; self . block . hash (state) ; } }
};
}
