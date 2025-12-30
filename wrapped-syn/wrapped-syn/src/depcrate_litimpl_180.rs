// Generated macro for impl_180 (impl)
macro_rules! Depcrate_litimpl_180 {
() => {
// Module: crate::lit
// Provides: {"impl_180"}
// Dependencies: {}
impl Hash for LitKind { fn hash < H : Hasher > (& self , hasher : & mut H) { match * self { LitKind :: Bool (b) => (0u8 , b) . hash (hasher) , LitKind :: Other (ref l) => (1u8 , l . to_string ()) . hash (hasher) , } } }
};
}
