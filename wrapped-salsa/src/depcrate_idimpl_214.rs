// Generated macro for impl_214 (impl)
macro_rules! Depcrate_idimpl_214 {
() => {
// Module: crate::id
// Provides: {"impl_214"}
// Dependencies: {}
impl Hash for Id { fn hash < H : Hasher > (& self , state : & mut H) { state . write_u64 (self . as_bits ()) ; } }
};
}
