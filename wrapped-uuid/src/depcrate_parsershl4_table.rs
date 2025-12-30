// Generated macro for SHL4_TABLE (const)
macro_rules! Depcrate_parserSHL4_TABLE {
() => {
// Module: crate::parser
// Provides: {"SHL4_TABLE"}
// Dependencies: {}
const SHL4_TABLE : & [u8 ; 256] = & { let mut buf = [0 ; 256] ; let mut i : u8 = 0 ; loop { buf [i as usize] = i . wrapping_shl (4) ; if i == 255 { break buf ; } i += 1 ; } } ;
};
}
