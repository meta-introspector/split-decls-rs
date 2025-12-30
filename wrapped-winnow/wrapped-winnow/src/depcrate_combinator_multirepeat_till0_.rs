// Generated macro for repeat_till0_ (function)
macro_rules! Depcrate_combinator_multirepeat_till0_ {
() => {
// Module: crate::combinator::multi
// Provides: {"repeat_till0_"}
// Dependencies: {}
fn repeat_till0_ < I , O , C , P , E , F , G > (f : & mut F , g : & mut G , i : & mut I) -> Result < (C , P) , E > where I : Stream , C : Accumulate < O > , F : Parser < I , O , E > , G : Parser < I , P , E > , E : ParserError < I > , { let mut res = C :: initial (None) ; loop { let start = i . checkpoint () ; let len = i . eof_offset () ; match g . parse_next (i) { Ok (o) => return Ok ((res , o)) , Err (e) if e . is_backtrack () => { i . reset (& start) ; match f . parse_next (i) { Err (e) => return Err (e . append (i , & start)) , Ok (o) => { if i . eof_offset () == len { return Err (ParserError :: assert (i , "`repeat` parsers must always consume" ,)) ; } res . accumulate (o) ; } } } Err (e) => return Err (e) , } } }
};
}
