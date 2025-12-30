// Generated macro for separated_n_ (function)
macro_rules! Depcrate_combinator_multiseparated_n_ {
() => {
// Module: crate::combinator::multi
// Provides: {"separated_n_"}
// Dependencies: {}
fn separated_n_ < I , O , C , O2 , E , P , S > (count : usize , parser : & mut P , separator : & mut S , input : & mut I ,) -> Result < C , E > where I : Stream , C : Accumulate < O > , P : Parser < I , O , E > , S : Parser < I , O2 , E > , E : ParserError < I > , { let mut acc = C :: initial (Some (count)) ; if count == 0 { return Ok (acc) ; } let start = input . checkpoint () ; match parser . parse_next (input) { Err (e) => { return Err (e . append (input , & start)) ; } Ok (o) => { acc . accumulate (o) ; } } for _ in 1 .. count { let start = input . checkpoint () ; let len = input . eof_offset () ; match separator . parse_next (input) { Err (e) => { return Err (e . append (input , & start)) ; } Ok (_) => { if input . eof_offset () == len { return Err (ParserError :: assert (input , "`separated` separator parser must always consume" ,)) ; } match parser . parse_next (input) { Err (e) => { return Err (e . append (input , & start)) ; } Ok (o) => { acc . accumulate (o) ; } } } } } Ok (acc) }
};
}
