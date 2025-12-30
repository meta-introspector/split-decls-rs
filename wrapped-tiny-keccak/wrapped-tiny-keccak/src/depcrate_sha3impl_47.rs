// Generated macro for impl_47 (impl)
macro_rules! Depcrate_sha3impl_47 {
() => {
// Module: crate::sha3
// Provides: {"impl_47"}
// Dependencies: {}
impl Sha3 { const DELIM : u8 = 0x06 ; # [doc = " Creates  new [`Sha3`] hasher with a security level of 224 bits."] # [doc = ""] # [doc = " [`Sha3`]: struct.Sha3.html"] pub fn v224 () -> Sha3 { Sha3 :: new (224) } # [doc = " Creates  new [`Sha3`] hasher with a security level of 256 bits."] # [doc = ""] # [doc = " [`Sha3`]: struct.Sha3.html"] pub fn v256 () -> Sha3 { Sha3 :: new (256) } # [doc = " Creates  new [`Sha3`] hasher with a security level of 384 bits."] # [doc = ""] # [doc = " [`Sha3`]: struct.Sha3.html"] pub fn v384 () -> Sha3 { Sha3 :: new (384) } # [doc = " Creates  new [`Sha3`] hasher with a security level of 512 bits."] # [doc = ""] # [doc = " [`Sha3`]: struct.Sha3.html"] pub fn v512 () -> Sha3 { Sha3 :: new (512) } fn new (bits : usize) -> Sha3 { Sha3 { state : KeccakState :: new (bits_to_rate (bits) , Self :: DELIM) , } } }
};
}
