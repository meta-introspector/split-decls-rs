// Generated macro for impl_45 (impl)
macro_rules! Depcrate_aes_ctrimpl_45 {
() => {
// Module: crate::aes_ctr
// Provides: {"impl_45"}
// Dependencies: {}
impl < C > AesCtrZipKeyStream < C > where C : AesKind , C :: Cipher : KeyInit , { # [doc = " Creates a new zip variant AES-CTR key stream."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This panics if `key` doesn't have the correct size for cipher `C`."] pub fn new (key : & [u8]) -> AesCtrZipKeyStream < C > { AesCtrZipKeyStream { counter : 1 , cipher : C :: Cipher :: new (GenericArray :: from_slice (key)) , buffer : [0u8 ; AES_BLOCK_SIZE] , pos : AES_BLOCK_SIZE , } } }
};
}
