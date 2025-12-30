// Generated macro for escaped_transform_internal (function)
macro_rules! Depcrate_asciiescaped_transform_internal {
() => {
// Module: crate::ascii
// Provides: {"escaped_transform_internal"}
// Dependencies: {}
fn escaped_transform_internal < I , Error , F , NormalOutput , G , EscapeOutput , Output , const PARTIAL : bool , > (input : & mut I , normal : & mut F , control_char : char , transform : & mut G ,) -> Result < Output , Error > where I : StreamIsPartial , I : Stream , I : Compare < char > , Output : crate :: stream :: Accumulate < NormalOutput > , Output : crate :: stream :: Accumulate < EscapeOutput > , F : Parser < I , NormalOutput , Error > , G : Parser < I , EscapeOutput , Error > , Error : ParserError < I > , { let mut res = < Output as crate :: stream :: Accumulate < NormalOutput > > :: initial (Some (input . eof_offset ())) ; while input . eof_offset () > 0 { let current_len = input . eof_offset () ; match opt (normal . by_ref ()) . parse_next (input) ? { Some (o) => { res . accumulate (o) ; if input . eof_offset () == current_len { return Err (ParserError :: assert (input , "`escaped_transform` parsers must always consume" ,)) ; } } None => { if opt (control_char) . parse_next (input) ? . is_some () { let o = transform . parse_next (input) ? ; res . accumulate (o) ; } else { return Ok (res) ; } } } } if PARTIAL && input . is_partial () { Err (ParserError :: incomplete (input , Needed :: Unknown)) } else { Ok (res) } }
};
}
