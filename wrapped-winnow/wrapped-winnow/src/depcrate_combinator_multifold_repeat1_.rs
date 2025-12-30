// Generated macro for fold_repeat1_ (function)
macro_rules! Depcrate_combinator_multifold_repeat1_ {
() => {
// Module: crate::combinator::multi
// Provides: {"fold_repeat1_"}
// Dependencies: {}
fn fold_repeat1_ < I , O , E , P , N , F , R > (parser : & mut P , init : & mut N , fold : & mut F , input : & mut I ,) -> Result < R , E > where I : Stream , P : Parser < I , O , E > , N : FnMut () -> R , F : FnMut (R , O) -> R , E : ParserError < I > , { let start = input . checkpoint () ; match parser . parse_next (input) { Err (err) => Err (err . append (input , & start)) , Ok (output) => { let init = init () ; let mut res = fold (init , output) ; loop { let start = input . checkpoint () ; let len = input . eof_offset () ; match parser . parse_next (input) { Err (err) if err . is_backtrack () => { input . reset (& start) ; break ; } Err (err) => return Err (err) , Ok (output) => { if input . eof_offset () == len { return Err (ParserError :: assert (input , "`repeat` parsers must always consume" ,)) ; } res = fold (res , output) ; } } } Ok (res) } } }
};
}
