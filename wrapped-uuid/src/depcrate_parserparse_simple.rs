// Generated macro for parse_simple (function)
macro_rules! Depcrate_parserparse_simple {
() => {
// Module: crate::parser
// Provides: {"parse_simple"}
// Dependencies: {}
# [inline] pub (crate) const fn parse_simple (s : & '_ [u8]) -> Result < [u8 ; 16] , InvalidUuid < '_ > > { if s . len () != 32 { return Err (InvalidUuid (s)) ; } let mut buf : [u8 ; 16] = [0 ; 16] ; let mut i = 0 ; while i < 16 { let h1 = HEX_TABLE [s [i * 2] as usize] ; let h2 = HEX_TABLE [s [i * 2 + 1] as usize] ; if h1 | h2 == 0xff { return Err (InvalidUuid (s)) ; } buf [i] = SHL4_TABLE [h1 as usize] | h2 ; i += 1 ; } Ok (buf) }
};
}
