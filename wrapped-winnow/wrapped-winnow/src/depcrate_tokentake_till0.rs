// Generated macro for take_till0 (function)
macro_rules! Depcrate_tokentake_till0 {
() => {
// Module: crate::token
// Provides: {"take_till0"}
// Dependencies: {}
fn take_till0 < P , I : StreamIsPartial + Stream , E : ParserError < I > , const PARTIAL : bool > (input : & mut I , predicate : P ,) -> Result < < I as Stream > :: Slice , E > where P : Fn (I :: Token) -> bool , { let offset = match input . offset_for (predicate) { Some (offset) => offset , None if PARTIAL && input . is_partial () => { return Err (ParserError :: incomplete (input , Needed :: new (1))) ; } None => input . eof_offset () , } ; Ok (input . next_slice (offset)) }
};
}
