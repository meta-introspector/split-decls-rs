// Generated macro for decode_shortu16_len (function)
macro_rules! Depcratedecode_shortu16_len {
() => {
// Module: crate
// Provides: {"decode_shortu16_len"}
// Dependencies: {}
# [doc = " Return the decoded value and how many bytes it consumed."] # [allow (clippy :: result_unit_err)] pub fn decode_shortu16_len (bytes : & [u8]) -> Result < (usize , usize) , () > { let mut val = 0 ; for (nth_byte , byte) in bytes . iter () . take (MAX_ENCODING_LENGTH) . enumerate () { match visit_byte (* byte , val , nth_byte) . map_err (| _ | ()) ? { VisitStatus :: More (new_val) => val = new_val , VisitStatus :: Done (new_val) => { return Ok ((usize :: from (new_val) , nth_byte . saturating_add (1))) ; } } } Err (()) }
};
}
