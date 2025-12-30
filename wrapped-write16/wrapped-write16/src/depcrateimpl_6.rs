// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
# [cfg (feature = "smallvec")] impl < A : smallvec :: Array < Item = u16 > > Write16 for smallvec :: SmallVec < A > { # [inline (always)] fn write_slice (& mut self , s : & [u16]) -> core :: fmt :: Result { self . extend_from_slice (s) ; Ok (()) } # [inline (always)] fn write_char (& mut self , c : char) -> core :: fmt :: Result { if c <= '\u{FFFF}' { self . push (c as u16) ; } else { let mut buf = [0u16 ; 2] ; let u = u32 :: from (c) ; buf [0] = (0xD7C0 + (u >> 10)) as u16 ; buf [1] = (0xDC00 + (u & 0x3FF)) as u16 ; self . extend_from_slice (& mut buf) ; } Ok (()) } # [inline (always)] fn size_hint (& mut self , upcoming : usize) -> core :: fmt :: Result { self . reserve (upcoming) ; Ok (()) } }
};
}
