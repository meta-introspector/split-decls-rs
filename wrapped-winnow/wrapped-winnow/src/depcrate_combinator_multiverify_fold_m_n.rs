// Generated macro for verify_fold_m_n (function)
macro_rules! Depcrate_combinator_multiverify_fold_m_n {
() => {
// Module: crate::combinator::multi
// Provides: {"verify_fold_m_n"}
// Dependencies: {}
fn verify_fold_m_n < I , O , E , P , N , F , R > (min : usize , max : usize , parse : & mut P , init : & mut N , fold : & mut F , input : & mut I ,) -> Result < R , E > where I : Stream , P : Parser < I , O , E > , N : FnMut () -> R , F : FnMut (R , O) -> Option < R > , E : ParserError < I > , { if min > max { return Err (ParserError :: assert (input , "range should be ascending, rather than descending" ,)) ; } let mut res = init () ; for count in 0 .. max { let start = input . checkpoint () ; let len = input . eof_offset () ; match parse . parse_next (input) { Ok (output) => { if input . eof_offset () == len { return Err (ParserError :: assert (input , "`repeat` parsers must always consume" ,)) ; } let Some (res_) = fold (res , output) else { input . reset (& start) ; let res = Err (ParserError :: from_input (input)) ; super :: debug :: trace_result ("verify_fold" , & res) ; return res ; } ; res = res_ ; } Err (err) if err . is_backtrack () => { if count < min { return Err (err . append (input , & start)) ; } else { input . reset (& start) ; break ; } } Err (err) => return Err (err) , } } Ok (res) }
};
}
