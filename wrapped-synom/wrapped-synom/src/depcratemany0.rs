// Generated macro for many0 (function)
macro_rules! Depcratemany0 {
() => {
// Module: crate
// Provides: {"many0"}
// Dependencies: {}
# [doc (hidden)] pub fn many0 < 'a , T > (mut input : Cursor , f : fn (Cursor) -> PResult < T >) -> PResult < Vec < T > > { let mut res = Vec :: new () ; loop { if input . eof () { return Ok ((input , res)) ; } match f (input) { Err (_) => { return Ok ((input , res)) ; } Ok ((i , o)) => { if i == input { return parse_error () ; } res . push (o) ; input = i ; } } } }
};
}
