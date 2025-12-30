// Generated macro for take_until1_ (function)
macro_rules! Depcrate_tokentake_until1_ {
() => {
// Module: crate::token
// Provides: {"take_until1_"}
// Dependencies: {}
fn take_until1_ < T , I , Error : ParserError < I > , const PARTIAL : bool > (i : & mut I , t : T ,) -> Result < < I as Stream > :: Slice , Error > where I : StreamIsPartial , I : Stream + FindSlice < T > , { match i . find_slice (t) { None if PARTIAL && i . is_partial () => Err (ParserError :: incomplete (i , Needed :: Unknown)) , None => Err (ParserError :: from_input (i)) , Some (range) => { if range . start == 0 { Err (ParserError :: from_input (i)) } else { Ok (i . next_slice (range . start)) } } } }
};
}
