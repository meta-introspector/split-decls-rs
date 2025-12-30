// Generated macro for try_fold_m_n (function)
macro_rules! Depcrate_combinator_multitry_fold_m_n {
() => {
// Module: crate::combinator::multi
// Provides: {"try_fold_m_n"}
// Dependencies: {}
fn try_fold_m_n < I , O , E , P , N , F , R , RE > (min : usize , max : usize , parse : & mut P , init : & mut N , fold : & mut F , input : & mut I ,) -> Result < R , E > where I : Stream , P : Parser < I , O , E > , N : FnMut () -> R , F : FnMut (R , O) -> Result < R , RE > , E : ParserError < I > + FromExternalError < I , RE > , { if min > max { return Err (ParserError :: assert (input , "range should be ascending, rather than descending" ,)) ; } let mut res = init () ; for count in 0 .. max { let start = input . checkpoint () ; let len = input . eof_offset () ; match parse . parse_next (input) { Ok (output) => { if input . eof_offset () == len { return Err (ParserError :: assert (input , "`repeat` parsers must always consume" ,)) ; } match fold (res , output) { Ok (res_) => res = res_ , Err (err) => { input . reset (& start) ; let res = Err (E :: from_external_error (input , err)) ; super :: debug :: trace_result ("try_fold" , & res) ; return res ; } } } Err (err) if err . is_backtrack () => { if count < min { return Err (err . append (input , & start)) ; } else { input . reset (& start) ; break ; } } Err (err) => return Err (err) , } } Ok (res) }
};
}
