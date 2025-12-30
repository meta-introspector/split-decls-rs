// Generated macro for take_until_m_n_ (function)
macro_rules! Depcrate_tokentake_until_m_n_ {
() => {
// Module: crate::token
// Provides: {"take_until_m_n_"}
// Dependencies: {}
fn take_until_m_n_ < T , I , Error : ParserError < I > , const PARTIAL : bool > (i : & mut I , start : usize , end : usize , t : T ,) -> Result < < I as Stream > :: Slice , Error > where I : StreamIsPartial , I : Stream + FindSlice < T > , { if end < start { return Err (ParserError :: assert (i , "`occurrences` should be ascending, rather than descending" ,)) ; } match i . find_slice (t) { Some (range) => { let start_offset = i . offset_at (start) ; let end_offset = i . offset_at (end) . unwrap_or_else (| _err | i . eof_offset ()) ; if start_offset . map (| s | range . start < s) . unwrap_or (true) { if PARTIAL && i . is_partial () { return Err (ParserError :: incomplete (i , Needed :: Unknown)) ; } else { return Err (ParserError :: from_input (i)) ; } } if end_offset < range . start { return Err (ParserError :: from_input (i)) ; } Ok (i . next_slice (range . start)) } None if PARTIAL && i . is_partial () => Err (ParserError :: incomplete (i , Needed :: Unknown)) , None => Err (ParserError :: from_input (i)) , } }
};
}
