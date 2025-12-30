// Generated macro for peek_bit (function)
macro_rules! Depcrate_streampeek_bit {
() => {
// Module: crate::stream
// Provides: {"peek_bit"}
// Dependencies: {}
fn peek_bit < I > (i : & (I , usize)) -> Option < bool > where I : Stream < Token = u8 > + Clone , { if i . eof_offset () == 0 { return None ; } let offset = i . 1 ; let mut next_i = i . 0 . clone () ; let byte = next_i . next_token () ? ; let bit = (byte >> offset) & 0x1 == 0x1 ; let next_offset = offset + 1 ; if next_offset == 8 { Some (bit) } else { Some (bit) } }
};
}
