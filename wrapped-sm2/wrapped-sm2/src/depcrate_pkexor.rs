// Generated macro for xor (function)
macro_rules! Depcrate_pkexor {
() => {
// Module: crate::pke
// Provides: {"xor"}
// Dependencies: {}
# [doc = " XORs a portion of the buffer `c2` with a hash value."] fn xor (c2 : & mut [u8] , ha : & [u8] , offset : usize , xor_len : usize) { for i in 0 .. xor_len { c2 [offset + i] ^= ha [i] ; } }
};
}
