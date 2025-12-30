// Generated macro for literal_ (function)
macro_rules! Depcrate_tokenliteral_ {
() => {
// Module: crate::token
// Provides: {"literal_"}
// Dependencies: {}
fn literal_ < T , I , Error : ParserError < I > , const PARTIAL : bool > (i : & mut I , t : T ,) -> Result < < I as Stream > :: Slice , Error > where I : StreamIsPartial , I : Stream + Compare < T > , T : core :: fmt :: Debug , { match i . compare (t) { CompareResult :: Ok (len) => Ok (i . next_slice (len)) , CompareResult :: Incomplete if PARTIAL && i . is_partial () => { Err (ParserError :: incomplete (i , Needed :: Unknown)) } CompareResult :: Incomplete | CompareResult :: Error => Err (ParserError :: from_input (i)) , } }
};
}
