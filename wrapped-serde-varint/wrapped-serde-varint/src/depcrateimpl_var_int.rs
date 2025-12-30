// Generated macro for impl_var_int (macro)
macro_rules! Depcrateimpl_var_int {
() => {
// Module: crate
// Provides: {"impl_var_int"}
// Dependencies: {}
macro_rules ! impl_var_int { ($ type : ty) => { impl VarInt for $ type { fn visit_seq <'de , A > (mut seq : A) -> Result < Self , A :: Error > where A : SeqAccess <'de >, { let mut out = 0 ; let mut shift = 0u32 ; while shift < <$ type >:: BITS { let Some (byte) = seq . next_element ::< u8 > () ? else { return Err (A :: Error :: custom ("Invalid Sequence")) ; } ; out |= ((byte & 0x7F) as Self) << shift ; if byte & 0x80 == 0 { if (out >> shift) as u8 != byte { return Err (A :: Error :: custom ("Last Byte Truncated")) ; } if byte == 0u8 && (shift != 0 || out != 0) { return Err (A :: Error :: custom ("Invalid Trailing Zeros")) ; } return Ok (out) ; } shift += 7 ; } Err (A :: Error :: custom ("Left Shift Overflows")) } fn serialize < S > (mut self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let bits = <$ type >:: BITS - self . leading_zeros () ; let num_bytes = bits . div_ceil (7) . max (1) as usize ; let mut seq = serializer . serialize_tuple (num_bytes) ?; while self >= 0x80 { let byte = ((self & 0x7F) | 0x80) as u8 ; seq . serialize_element (& byte) ?; self >>= 7 ; } seq . serialize_element (& (self as u8)) ?; seq . end () } } } ; }
};
}
