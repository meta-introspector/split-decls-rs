// Generated macro for impl_259 (impl)
macro_rules! Depcrate_symbolimpl_259 {
() => {
// Module: crate::symbol
// Provides: {"impl_259"}
// Dependencies: {}
impl Hash for Ident { fn hash < H : Hasher > (& self , state : & mut H) { self . name . hash (state) ; self . span . ctxt () . hash (state) ; } }
};
}
