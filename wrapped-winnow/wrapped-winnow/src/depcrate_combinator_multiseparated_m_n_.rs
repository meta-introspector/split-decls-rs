// Generated macro for separated_m_n_ (function)
macro_rules! Depcrate_combinator_multiseparated_m_n_ {
() => {
// Module: crate::combinator::multi
// Provides: {"separated_m_n_"}
// Dependencies: {}
fn separated_m_n_ < I , O , C , O2 , E , P , S > (min : usize , max : usize , parser : & mut P , separator : & mut S , input : & mut I ,) -> Result < C , E > where I : Stream , C : Accumulate < O > , P : Parser < I , O , E > , S : Parser < I , O2 , E > , E : ParserError < I > , { if min > max { return Err (ParserError :: assert (input , "range should be ascending, rather than descending" ,)) ; } let mut acc = C :: initial (Some (min)) ; let start = input . checkpoint () ; match parser . parse_next (input) { Err (e) if e . is_backtrack () => { if min == 0 { input . reset (& start) ; return Ok (acc) ; } else { return Err (e . append (input , & start)) ; } } Err (e) => return Err (e) , Ok (o) => { acc . accumulate (o) ; } } for index in 1 .. max { let start = input . checkpoint () ; let len = input . eof_offset () ; match separator . parse_next (input) { Err (e) if e . is_backtrack () => { if index < min { return Err (e . append (input , & start)) ; } else { input . reset (& start) ; return Ok (acc) ; } } Err (e) => { return Err (e) ; } Ok (_) => { if input . eof_offset () == len { return Err (ParserError :: assert (input , "`separated` separator parser must always consume" ,)) ; } match parser . parse_next (input) { Err (e) if e . is_backtrack () => { if index < min { return Err (e . append (input , & start)) ; } else { input . reset (& start) ; return Ok (acc) ; } } Err (e) => { return Err (e) ; } Ok (o) => { acc . accumulate (o) ; } } } } } Ok (acc) }
};
}
