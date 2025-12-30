// Generated macro for impl_14 (impl)
macro_rules! Depcrate_bytearrayimpl_14 {
() => {
// Module: crate::bytearray
// Provides: {"impl_14"}
// Dependencies: {}
impl < const N : usize > Debug for ByteArray < N > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Debug :: fmt (& self . bytes , f) } }
};
}
