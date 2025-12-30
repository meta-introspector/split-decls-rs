// Generated macro for impl_62 (impl)
macro_rules! Depcrate_sip128impl_62 {
() => {
// Module: crate::sip128
// Provides: {"impl_62"}
// Dependencies: {}
impl < S : Sip > hash :: Hasher for Hasher < S > { # [inline] fn write_usize (& mut self , i : usize) { self . short_write (i , i . to_le () as u64) ; } # [inline] fn write_u8 (& mut self , i : u8) { self . short_write (i , i as u64) ; } # [inline] fn write_u32 (& mut self , i : u32) { self . short_write (i , i . to_le () as u64) ; } # [inline] fn write_u64 (& mut self , i : u64) { self . short_write (i , i . to_le ()) ; } # [inline] fn write (& mut self , msg : & [u8]) { let length = msg . len () ; self . length += length ; let mut needed = 0 ; if self . ntail != 0 { needed = 8 - self . ntail ; self . tail |= unsafe { u8to64_le (msg , 0 , cmp :: min (length , needed)) } << (8 * self . ntail) ; if length < needed { self . ntail += length ; return ; } else { self . state . v3 ^= self . tail ; S :: c_rounds (& mut self . state) ; self . state . v0 ^= self . tail ; self . ntail = 0 ; } } let len = length - needed ; let left = len & 0x7 ; let mut i = needed ; while i < len - left { let mi = unsafe { load_int_le ! (msg , i , u64) } ; self . state . v3 ^= mi ; S :: c_rounds (& mut self . state) ; self . state . v0 ^= mi ; i += 8 ; } self . tail = unsafe { u8to64_le (msg , i , left) } ; self . ntail = left ; } # [inline] fn finish (& self) -> u64 { self . finish128 () . h2 } }
};
}
