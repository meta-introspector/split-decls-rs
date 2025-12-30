// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
# [cfg (feature = "arrayvec")] impl < const CAP : usize > Write16 for arrayvec :: ArrayVec < u16 , CAP > { # [inline (always)] fn write_slice (& mut self , s : & [u16]) -> core :: fmt :: Result { if self . try_extend_from_slice (s) . is_ok () { Ok (()) } else { Err (core :: fmt :: Error { }) } } # [inline (always)] fn write_char (& mut self , c : char) -> core :: fmt :: Result { if c <= '\u{FFFF}' { if self . try_push (c as u16) . is_ok () { Ok (()) } else { Err (core :: fmt :: Error { }) } } else { let mut buf = [0u16 ; 2] ; let u = u32 :: from (c) ; buf [0] = (0xD7C0 + (u >> 10)) as u16 ; buf [1] = (0xDC00 + (u & 0x3FF)) as u16 ; self . write_slice (& mut buf) } } }
};
}
