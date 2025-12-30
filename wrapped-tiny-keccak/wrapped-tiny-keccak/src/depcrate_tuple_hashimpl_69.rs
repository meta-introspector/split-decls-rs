// Generated macro for impl_69 (impl)
macro_rules! Depcrate_tuple_hashimpl_69 {
() => {
// Module: crate::tuple_hash
// Provides: {"impl_69"}
// Dependencies: {}
impl TupleHash { # [doc = " Creates  new [`TupleHash`] hasher with a security level of 128 bits."] # [doc = ""] # [doc = " [`TupleHash`]: struct.TupleHash.html"] pub fn v128 (custom_string : & [u8]) -> TupleHash { TupleHash :: new (custom_string , 128) } # [doc = " Creates  new [`TupleHash`] hasher with a security level of 256 bits."] # [doc = ""] # [doc = " [`TupleHash`]: struct.TupleHash.html"] pub fn v256 (custom_string : & [u8]) -> TupleHash { TupleHash :: new (custom_string , 256) } fn new (custom_string : & [u8] , bits : usize) -> TupleHash { TupleHash { state : CShake :: new (b"TupleHash" , custom_string , bits) , } } }
};
}
