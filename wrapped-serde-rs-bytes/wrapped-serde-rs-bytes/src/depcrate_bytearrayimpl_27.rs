// Generated macro for impl_27 (impl)
macro_rules! Depcrate_bytearrayimpl_27 {
() => {
// Module: crate::bytearray
// Provides: {"impl_27"}
// Dependencies: {}
impl < const N : usize > Hash for ByteArray < N > { fn hash < H : Hasher > (& self , state : & mut H) { self . bytes . hash (state) ; } }
};
}
