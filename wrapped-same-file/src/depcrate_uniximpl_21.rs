// Generated macro for impl_21 (impl)
macro_rules! Depcrate_uniximpl_21 {
() => {
// Module: crate::unix
// Provides: {"impl_21"}
// Dependencies: {}
impl Hash for Handle { fn hash < H : Hasher > (& self , state : & mut H) { self . dev . hash (state) ; self . ino . hash (state) ; } }
};
}
