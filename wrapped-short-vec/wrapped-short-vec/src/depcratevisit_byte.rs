// Generated macro for visit_byte (function)
macro_rules! Depcratevisit_byte {
() => {
// Module: crate
// Provides: {"visit_byte"}
// Dependencies: {}
fn visit_byte (elem : u8 , val : u16 , nth_byte : usize) -> VisitResult { if elem == 0 && nth_byte != 0 { return Err (VisitError :: Alias) ; } let val = u32 :: from (val) ; let elem = u32 :: from (elem) ; let elem_val = elem & 0x7f ; let elem_done = (elem & 0x80) == 0 ; if nth_byte >= MAX_ENCODING_LENGTH { return Err (VisitError :: TooLong (nth_byte . saturating_add (1))) ; } else if nth_byte == MAX_ENCODING_LENGTH . saturating_sub (1) && ! elem_done { return Err (VisitError :: ByteThreeContinues) ; } let shift = u32 :: try_from (nth_byte) . unwrap_or (u32 :: MAX) . saturating_mul (7) ; let elem_val = elem_val . checked_shl (shift) . unwrap_or (u32 :: MAX) ; let new_val = val | elem_val ; let val = u16 :: try_from (new_val) . map_err (| _ | VisitError :: Overflow (new_val)) ? ; if elem_done { Ok (VisitStatus :: Done (val)) } else { Ok (VisitStatus :: More (val)) } }
};
}
