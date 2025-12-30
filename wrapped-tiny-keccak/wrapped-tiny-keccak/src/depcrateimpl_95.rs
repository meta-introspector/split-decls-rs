// Generated macro for impl_95 (impl)
macro_rules! Depcrateimpl_95 {
() => {
// Module: crate
// Provides: {"impl_95"}
// Dependencies: {}
impl Buffer { fn words (& mut self) -> & mut [u64 ; WORDS] { & mut self . 0 } # [cfg (target_endian = "little")] # [inline] fn execute < F : FnOnce (& mut [u8]) > (& mut self , offset : usize , len : usize , f : F) { let buffer : & mut [u8 ; WORDS * 8] = unsafe { core :: mem :: transmute (& mut self . 0) } ; f (& mut buffer [offset ..] [.. len]) ; } # [cfg (target_endian = "big")] # [inline] fn execute < F : FnOnce (& mut [u8]) > (& mut self , offset : usize , len : usize , f : F) { fn swap_endianess (buffer : & mut [u64]) { for item in buffer { * item = item . swap_bytes () ; } } let start = offset / 8 ; let end = (offset + len + 7) / 8 ; swap_endianess (& mut self . 0 [start .. end]) ; let buffer : & mut [u8 ; WORDS * 8] = unsafe { core :: mem :: transmute (& mut self . 0) } ; f (& mut buffer [offset ..] [.. len]) ; swap_endianess (& mut self . 0 [start .. end]) ; } fn setout (& mut self , dst : & mut [u8] , offset : usize , len : usize) { self . execute (offset , len , | buffer | dst [.. len] . copy_from_slice (buffer)) ; } fn xorin (& mut self , src : & [u8] , offset : usize , len : usize) { self . execute (offset , len , | dst | { assert ! (dst . len () <= src . len ()) ; let len = dst . len () ; let mut dst_ptr = dst . as_mut_ptr () ; let mut src_ptr = src . as_ptr () ; for _ in 0 .. len { unsafe { * dst_ptr ^= * src_ptr ; src_ptr = src_ptr . offset (1) ; dst_ptr = dst_ptr . offset (1) ; } } }) ; } fn pad (& mut self , offset : usize , delim : u8 , rate : usize) { self . execute (offset , 1 , | buff | buff [0] ^= delim) ; self . execute (rate - 1 , 1 , | buff | buff [0] ^= 0x80) ; } }
};
}
