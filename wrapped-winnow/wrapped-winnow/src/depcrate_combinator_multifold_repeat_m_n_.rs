// Generated macro for fold_repeat_m_n_ (function)
macro_rules! Depcrate_combinator_multifold_repeat_m_n_ {
() => {
// Module: crate::combinator::multi
// Provides: {"fold_repeat_m_n_"}
// Dependencies: {}
fn fold_repeat_m_n_ < I , O , E , P , N , F , R > (min : usize , max : usize , parse : & mut P , init : & mut N , fold : & mut F , input : & mut I ,) -> Result < R , E > where I : Stream , P : Parser < I , O , E > , N : FnMut () -> R , F : FnMut (R , O) -> R , E : ParserError < I > , { if min > max { return Err (ParserError :: assert (input , "range should be ascending, rather than descending" ,)) ; } let mut res = init () ; for count in 0 .. max { let start = input . checkpoint () ; let len = input . eof_offset () ; match parse . parse_next (input) { Ok (output) => { if input . eof_offset () == len { return Err (ParserError :: assert (input , "`repeat` parsers must always consume" ,)) ; } res = fold (res , output) ; } Err (err) if err . is_backtrack () => { if count < min { return Err (err . append (input , & start)) ; } else { input . reset (& start) ; break ; } } Err (err) => return Err (err) , } } Ok (res) }
};
}
