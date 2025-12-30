// Generated macro for any_ (function)
macro_rules! Depcrate_tokenany_ {
() => {
// Module: crate::token
// Provides: {"any_"}
// Dependencies: {}
fn any_ < I , E : ParserError < I > , const PARTIAL : bool > (input : & mut I) -> Result < < I as Stream > :: Token , E > where I : StreamIsPartial , I : Stream , { input . next_token () . ok_or_else (| | { if PARTIAL && input . is_partial () { ParserError :: incomplete (input , Needed :: new (1)) } else { ParserError :: from_input (input) } }) }
};
}
