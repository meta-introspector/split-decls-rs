// Generated macro for parse_simple (function)
macro_rules! Depcrate_layer_idparse_simple {
() => {
// Module: crate::layer::id
// Provides: {"parse_simple"}
// Dependencies: {}
# [inline] const fn parse_simple (s : & [u8]) -> Option < Uuid > { if s . len () != 32 { return None ; } let mut buf : [u8 ; 16] = [0 ; 16] ; let mut i = 0 ; while i < 16 { let h1 = HEX_TABLE [s [i * 2] as usize] ; let h2 = HEX_TABLE [s [i * 2 + 1] as usize] ; if h1 | h2 == 0xff { return None ; } buf [i] = SHL4_TABLE [h1 as usize] | h2 ; i += 1 ; } Some (Uuid :: from_bytes (buf)) }
};
}
