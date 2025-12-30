// Generated macro for repeat_till_m_n_ (function)
macro_rules! Depcrate_combinator_multirepeat_till_m_n_ {
() => {
// Module: crate::combinator::multi
// Provides: {"repeat_till_m_n_"}
// Dependencies: {}
fn repeat_till_m_n_ < I , O , C , P , E , F , G > (min : usize , max : usize , f : & mut F , g : & mut G , i : & mut I ,) -> Result < (C , P) , E > where I : Stream , C : Accumulate < O > , F : Parser < I , O , E > , G : Parser < I , P , E > , E : ParserError < I > , { if min > max { return Err (ParserError :: assert (i , "range should be ascending, rather than descending" ,)) ; } let mut res = C :: initial (Some (min)) ; let start = i . checkpoint () ; for _ in 0 .. min { match f . parse_next (i) { Ok (o) => { res . accumulate (o) ; } Err (e) => { return Err (e . append (i , & start)) ; } } } for count in min ..= max { let start = i . checkpoint () ; let len = i . eof_offset () ; match g . parse_next (i) { Ok (o) => return Ok ((res , o)) , Err (err) if err . is_backtrack () => { if count == max { return Err (err) ; } i . reset (& start) ; match f . parse_next (i) { Err (e) => { return Err (e . append (i , & start)) ; } Ok (o) => { if i . eof_offset () == len { return Err (ParserError :: assert (i , "`repeat` parsers must always consume" ,)) ; } res . accumulate (o) ; } } } Err (e) => return Err (e) , } } unreachable ! () }
};
}
