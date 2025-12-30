// Generated macro for fold_repeat0_ (function)
macro_rules! Depcrate_combinator_multifold_repeat0_ {
() => {
// Module: crate::combinator::multi
// Provides: {"fold_repeat0_"}
// Dependencies: {}
fn fold_repeat0_ < I , O , E , P , N , F , R > (parser : & mut P , init : & mut N , fold : & mut F , input : & mut I ,) -> Result < R , E > where I : Stream , P : Parser < I , O , E > , N : FnMut () -> R , F : FnMut (R , O) -> R , E : ParserError < I > , { let mut res = init () ; loop { let start = input . checkpoint () ; let len = input . eof_offset () ; match parser . parse_next (input) { Ok (output) => { if input . eof_offset () == len { return Err (ParserError :: assert (input , "`repeat` parsers must always consume" ,)) ; } res = fold (res , output) ; } Err (err) if err . is_backtrack () => { input . reset (& start) ; return Ok (res) ; } Err (err) => { return Err (err) ; } } } }
};
}
