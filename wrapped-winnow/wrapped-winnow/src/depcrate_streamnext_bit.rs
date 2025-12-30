// Generated macro for next_bit (function)
macro_rules! Depcrate_streamnext_bit {
() => {
// Module: crate::stream
// Provides: {"next_bit"}
// Dependencies: {}
fn next_bit < I > (i : & mut (I , usize)) -> Option < bool > where I : Stream < Token = u8 > + Clone , { if i . eof_offset () == 0 { return None ; } let offset = i . 1 ; let mut next_i = i . 0 . clone () ; let byte = next_i . next_token () ? ; let bit = (byte >> offset) & 0x1 == 0x1 ; let next_offset = offset + 1 ; if next_offset == 8 { i . 0 = next_i ; i . 1 = 0 ; Some (bit) } else { i . 1 = next_offset ; Some (bit) } }
};
}
