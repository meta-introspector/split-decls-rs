// Generated macro for fold_repeat_n_ (function)
macro_rules! Depcrate_combinator_multifold_repeat_n_ {
() => {
// Module: crate::combinator::multi
// Provides: {"fold_repeat_n_"}
// Dependencies: {}
fn fold_repeat_n_ < I , O , E , P , N , F , R > (count : usize , parse : & mut P , init : & mut N , fold : & mut F , input : & mut I ,) -> Result < R , E > where I : Stream , P : Parser < I , O , E > , N : FnMut () -> R , F : FnMut (R , O) -> R , E : ParserError < I > , { let mut res = init () ; for _ in 0 .. count { let start = input . checkpoint () ; let len = input . eof_offset () ; match parse . parse_next (input) { Ok (output) => { if input . eof_offset () == len { return Err (ParserError :: assert (input , "`repeat` parsers must always consume" ,)) ; } res = fold (res , output) ; } Err (err) => { return Err (err . append (input , & start)) ; } } } Ok (res) }
};
}
