// Generated macro for escaped_internal (function)
macro_rules! Depcrate_asciiescaped_internal {
() => {
// Module: crate::ascii
// Provides: {"escaped_internal"}
// Dependencies: {}
fn escaped_internal < I , Error , F , G , O1 , O2 , const PARTIAL : bool > (input : & mut I , normal : & mut F , control_char : char , escapable : & mut G ,) -> Result < < I as Stream > :: Slice , Error > where I : StreamIsPartial , I : Stream , I : Compare < char > , F : Parser < I , O1 , Error > , G : Parser < I , O2 , Error > , Error : ParserError < I > , { let start = input . checkpoint () ; while input . eof_offset () > 0 { let current_len = input . eof_offset () ; match opt (normal . by_ref ()) . parse_next (input) ? { Some (_) => { if input . eof_offset () == current_len { return Err (ParserError :: assert (input , "`take_escaped` parsers must always consume" ,)) ; } } None => { if opt (control_char) . parse_next (input) ? . is_some () { let _ = escapable . parse_next (input) ? ; } else { let offset = input . offset_from (& start) ; input . reset (& start) ; return Ok (input . next_slice (offset)) ; } } } } if PARTIAL && input . is_partial () { Err (ParserError :: incomplete (input , Needed :: Unknown)) } else { input . reset (& start) ; Ok (input . finish ()) } }
};
}
