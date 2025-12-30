// Generated macro for check_is (macro)
macro_rules! Depcrate_asciicheck_is {
() => {
// Module: crate::ascii
// Provides: {"check_is"}
// Dependencies: {}
macro_rules ! check_is { ($ self : ident , $ check_int : ident , $ check_u8 : ident) => { if N <= 4 { Aligned4 :: from_ascii_bytes (&$ self . bytes) .$ check_int () } else if N <= 8 { Aligned8 :: from_ascii_bytes (&$ self . bytes) .$ check_int () } else { let mut i = 0 ; while i < N && $ self . bytes [i] as u8 != AsciiByte :: B0 as u8 { if ! ($ self . bytes [i] as u8) .$ check_u8 () { return false ; } i += 1 ; } true } } ; ($ self : ident , $ check_int : ident , !$ check_u8_0_inv : ident , !$ check_u8_1_inv : ident) => { if N <= 4 { Aligned4 :: from_ascii_bytes (&$ self . bytes) .$ check_int () } else if N <= 8 { Aligned8 :: from_ascii_bytes (&$ self . bytes) .$ check_int () } else { if ($ self . bytes [0] as u8) .$ check_u8_0_inv () { return false ; } let mut i = 1 ; while i < N && $ self . bytes [i] as u8 != AsciiByte :: B0 as u8 { if ($ self . bytes [i] as u8) .$ check_u8_1_inv () { return false ; } i += 1 ; } true } } ; ($ self : ident , $ check_int : ident , $ check_u8_0_inv : ident , $ check_u8_1_inv : ident) => { if N <= 4 { Aligned4 :: from_ascii_bytes (&$ self . bytes) .$ check_int () } else if N <= 8 { Aligned8 :: from_ascii_bytes (&$ self . bytes) .$ check_int () } else { if ! ($ self . bytes [0] as u8) .$ check_u8_0_inv () { return false ; } let mut i = 1 ; while i < N && $ self . bytes [i] as u8 != AsciiByte :: B0 as u8 { if ! ($ self . bytes [i] as u8) .$ check_u8_1_inv () { return false ; } i += 1 ; } true } } ; }
};
}
