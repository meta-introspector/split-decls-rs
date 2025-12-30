// Generated macro for separated1_ (function)
macro_rules! Depcrate_combinator_multiseparated1_ {
() => {
// Module: crate::combinator::multi
// Provides: {"separated1_"}
// Dependencies: {}
fn separated1_ < I , O , C , O2 , E , P , S > (parser : & mut P , separator : & mut S , input : & mut I ,) -> Result < C , E > where I : Stream , C : Accumulate < O > , P : Parser < I , O , E > , S : Parser < I , O2 , E > , E : ParserError < I > , { let mut acc = C :: initial (None) ; match parser . parse_next (input) { Err (e) => return Err (e) , Ok (o) => { acc . accumulate (o) ; } } loop { let start = input . checkpoint () ; let len = input . eof_offset () ; match separator . parse_next (input) { Err (e) if e . is_backtrack () => { input . reset (& start) ; return Ok (acc) ; } Err (e) => return Err (e) , Ok (_) => { if input . eof_offset () == len { return Err (ParserError :: assert (input , "`separated` separator parser must always consume" ,)) ; } match parser . parse_next (input) { Err (e) if e . is_backtrack () => { input . reset (& start) ; return Ok (acc) ; } Err (e) => return Err (e) , Ok (o) => { acc . accumulate (o) ; } } } } } }
};
}
