// Generated macro for take_till_m_n (function)
macro_rules! Depcrate_tokentake_till_m_n {
() => {
// Module: crate::token
// Provides: {"take_till_m_n"}
// Dependencies: {}
fn take_till_m_n < P , I , Error : ParserError < I > , const PARTIAL : bool > (input : & mut I , m : usize , n : usize , predicate : P ,) -> Result < < I as Stream > :: Slice , Error > where I : StreamIsPartial , I : Stream , P : Fn (I :: Token) -> bool , { if n < m { return Err (ParserError :: assert (input , "`occurrences` should be ascending, rather than descending" ,)) ; } let mut final_count = 0 ; for (processed , (offset , token)) in input . iter_offsets () . enumerate () { if predicate (token) { if processed < m { return Err (ParserError :: from_input (input)) ; } else { return Ok (input . next_slice (offset)) ; } } else { if processed == n { return Ok (input . next_slice (offset)) ; } final_count = processed + 1 ; } } if PARTIAL && input . is_partial () { if final_count == n { Ok (input . finish ()) } else { let needed = if m > input . eof_offset () { m - input . eof_offset () } else { 1 } ; Err (ParserError :: incomplete (input , Needed :: new (needed))) } } else { if m <= final_count { Ok (input . finish ()) } else { Err (ParserError :: from_input (input)) } } }
};
}
