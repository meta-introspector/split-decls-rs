// Generated macro for tokenize (function)
macro_rules! Depcrate_lexertokenize {
() => {
// Module: crate::lexer
// Provides: {"tokenize"}
// Dependencies: {}
pub (crate) fn tokenize (mut input : & str) -> Result < Vec < Token > > { let mut res = Vec :: new () ; let mut loc = Location :: default () ; while ! input . is_empty () { let old_input = input ; skip_ws (& mut input) ; skip_comment (& mut input) ; if old_input . len () == input . len () { match advance (& mut input) { Ok (kind) => { res . push (Token { kind , loc }) ; } Err (err) => return Err (err . with_location (loc)) , } } let consumed = old_input . len () - input . len () ; loc . advance (& old_input [.. consumed]) ; } Ok (res) }
};
}
