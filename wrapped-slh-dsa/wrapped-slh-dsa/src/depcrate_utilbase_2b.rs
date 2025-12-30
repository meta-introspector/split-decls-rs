// Generated macro for base_2b (function)
macro_rules! Depcrate_utilbase_2b {
() => {
// Module: crate::util
// Provides: {"base_2b"}
// Dependencies: {}
pub (crate) fn base_2b < OutLen : ArraySize , B : Unsigned > (x : & [u8]) -> Array < u16 , OutLen > { debug_assert ! (x . len () >= (OutLen :: USIZE * B :: USIZE) . div_ceil (8)) ; debug_assert ! (B :: USIZE <= 16) ; let mut bits = 0usize ; let mut i = 0 ; let mut total = 0usize ; Array :: < u16 , OutLen > :: from_fn (| _ : usize | { while bits < B :: USIZE { total = (total << 8) + x [i] as usize ; bits += 8 ; i += 1 ; } bits -= B :: USIZE ; let out = (total >> bits) & ((1 << B :: U8) - 1) ; total &= (1 << bits) - 1 ; out . try_into () . expect ("B is less than 16") }) }
};
}
