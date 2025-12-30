// Generated macro for take_until0_ (function)
macro_rules! Depcrate_tokentake_until0_ {
() => {
// Module: crate::token
// Provides: {"take_until0_"}
// Dependencies: {}
fn take_until0_ < T , I , Error : ParserError < I > , const PARTIAL : bool > (i : & mut I , t : T ,) -> Result < < I as Stream > :: Slice , Error > where I : StreamIsPartial , I : Stream + FindSlice < T > , { match i . find_slice (t) { Some (range) => Ok (i . next_slice (range . start)) , None if PARTIAL && i . is_partial () => Err (ParserError :: incomplete (i , Needed :: Unknown)) , None => Err (ParserError :: from_input (i)) , } }
};
}
