// Generated macro for impl_40 (impl)
macro_rules! Depcrate_shakeimpl_40 {
() => {
// Module: crate::shake
// Provides: {"impl_40"}
// Dependencies: {}
impl Shake { const DELIM : u8 = 0x1f ; # [doc = " Creates  new [`Shake`] hasher with a security level of 128 bits."] # [doc = ""] # [doc = " [`Shake`]: struct.Shake.html"] pub fn v128 () -> Shake { Shake :: new (128) } # [doc = " Creates  new [`Shake`] hasher with a security level of 256 bits."] # [doc = ""] # [doc = " [`Shake`]: struct.Shake.html"] pub fn v256 () -> Shake { Shake :: new (256) } pub (crate) fn new (bits : usize) -> Shake { Shake { state : KeccakState :: new (bits_to_rate (bits) , Self :: DELIM) , } } }
};
}
