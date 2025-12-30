// Generated macro for impl_34 (impl)
macro_rules! Depcrate_keccakimpl_34 {
() => {
// Module: crate::keccak
// Provides: {"impl_34"}
// Dependencies: {}
impl Keccak { const DELIM : u8 = 0x01 ; # [doc = " Creates  new [`Keccak`] hasher with a security level of 224 bits."] # [doc = ""] # [doc = " [`Keccak`]: struct.Keccak.html"] pub fn v224 () -> Keccak { Keccak :: new (224) } # [doc = " Creates  new [`Keccak`] hasher with a security level of 256 bits."] # [doc = ""] # [doc = " [`Keccak`]: struct.Keccak.html"] pub fn v256 () -> Keccak { Keccak :: new (256) } # [doc = " Creates  new [`Keccak`] hasher with a security level of 384 bits."] # [doc = ""] # [doc = " [`Keccak`]: struct.Keccak.html"] pub fn v384 () -> Keccak { Keccak :: new (384) } # [doc = " Creates  new [`Keccak`] hasher with a security level of 512 bits."] # [doc = ""] # [doc = " [`Keccak`]: struct.Keccak.html"] pub fn v512 () -> Keccak { Keccak :: new (512) } fn new (bits : usize) -> Keccak { Keccak { state : KeccakState :: new (bits_to_rate (bits) , Self :: DELIM) , } } }
};
}
