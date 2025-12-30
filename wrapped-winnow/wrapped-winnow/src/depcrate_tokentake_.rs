// Generated macro for take_ (function)
macro_rules! Depcrate_tokentake_ {
() => {
// Module: crate::token
// Provides: {"take_"}
// Dependencies: {}
fn take_ < I , Error : ParserError < I > , const PARTIAL : bool > (i : & mut I , c : usize ,) -> Result < < I as Stream > :: Slice , Error > where I : StreamIsPartial , I : Stream , { match i . offset_at (c) { Ok (offset) => Ok (i . next_slice (offset)) , Err (e) if PARTIAL && i . is_partial () => Err (ParserError :: incomplete (i , e)) , Err (_needed) => Err (ParserError :: from_input (i)) , } }
};
}
