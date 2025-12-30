// Generated macro for impl_61 (impl)
macro_rules! Depcrate_sip128impl_61 {
() => {
// Module: crate::sip128
// Provides: {"impl_61"}
// Dependencies: {}
impl hash :: Hasher for SipHasher24 { # [inline] fn write (& mut self , msg : & [u8]) { self . hasher . write (msg) } # [inline] fn finish (& self) -> u64 { self . hasher . finish () } # [inline] fn write_usize (& mut self , i : usize) { self . hasher . write_usize (i) ; } # [inline] fn write_u8 (& mut self , i : u8) { self . hasher . write_u8 (i) ; } # [inline] fn write_u16 (& mut self , i : u16) { self . hasher . write_u16 (i) ; } # [inline] fn write_u32 (& mut self , i : u32) { self . hasher . write_u32 (i) ; } # [inline] fn write_u64 (& mut self , i : u64) { self . hasher . write_u64 (i) ; } }
};
}
