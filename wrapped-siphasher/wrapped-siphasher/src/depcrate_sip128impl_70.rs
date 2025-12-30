// Generated macro for impl_70 (impl)
macro_rules! Depcrate_sip128impl_70 {
() => {
// Module: crate::sip128
// Provides: {"impl_70"}
// Dependencies: {}
impl Hash128 { # [doc = " Convert into a 16-bytes vector"] pub fn as_bytes (& self) -> [u8 ; 16] { let mut bytes = [0u8 ; 16] ; bytes [0 .. 8] . copy_from_slice (& self . h1 . to_le_bytes ()) ; bytes [8 .. 16] . copy_from_slice (& self . h2 . to_le_bytes ()) ; bytes } # [doc = " Convert into a `u128`"] # [inline] pub fn as_u128 (& self) -> u128 { let h1 = self . h1 . to_le () ; let h2 = self . h2 . to_le () ; h1 as u128 | ((h2 as u128) << 64) } # [doc = " Convert into `(u64, u64)`"] # [inline] pub fn as_u64 (& self) -> (u64 , u64) { let h1 = self . h1 . to_le () ; let h2 = self . h2 . to_le () ; (h1 , h2) } }
};
}
