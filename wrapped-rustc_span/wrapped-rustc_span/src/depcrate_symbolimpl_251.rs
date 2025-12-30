// Generated macro for impl_251 (impl)
macro_rules! Depcrate_symbolimpl_251 {
() => {
// Module: crate::symbol
// Provides: {"impl_251"}
// Dependencies: {}
impl Hash for Ident { fn hash < H : Hasher > (& self , state : & mut H) { self . name . hash (state) ; self . span . ctxt () . hash (state) ; } }
};
}
