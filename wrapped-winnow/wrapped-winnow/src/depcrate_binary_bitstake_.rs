// Generated macro for take_ (function)
macro_rules! Depcrate_binary_bitstake_ {
() => {
// Module: crate::binary::bits
// Provides: {"take_"}
// Dependencies: {}
fn take_ < I , O , E : ParserError < (I , usize) > , const PARTIAL : bool > (bit_input : & mut (I , usize) , count : usize ,) -> Result < O , E > where I : StreamIsPartial , I : Stream < Token = u8 > + Clone , O : From < u8 > + AddAssign + Shl < usize , Output = O > + Shr < usize , Output = O > , { if count == 0 { Ok (0u8 . into ()) } else { let (mut input , bit_offset) = bit_input . clone () ; if input . eof_offset () * BYTE < count + bit_offset { if PARTIAL && input . is_partial () { Err (ParserError :: incomplete (bit_input , Needed :: new (count))) } else { Err (ParserError :: from_input (& (input , bit_offset))) } } else { let cnt = (count + bit_offset) . div (BYTE) ; let mut acc : O = 0_u8 . into () ; let mut offset : usize = bit_offset ; let mut remaining : usize = count ; let mut end_offset : usize = 0 ; for (_ , byte) in input . iter_offsets () . take (cnt + 1) { if remaining == 0 { break ; } let val : O = if offset == 0 { byte . into () } else { (byte << offset >> offset) . into () } ; if remaining < BYTE - offset { acc += val >> (BYTE - offset - remaining) ; end_offset = remaining + offset ; break ; } else { acc += val << (remaining - (BYTE - offset)) ; remaining -= BYTE - offset ; offset = 0 ; } } let _ = input . next_slice (cnt) ; * bit_input = (input , end_offset) ; Ok (acc) } } }
};
}
