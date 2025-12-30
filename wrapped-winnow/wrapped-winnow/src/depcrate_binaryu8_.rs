// Generated macro for u8_ (function)
macro_rules! Depcrate_binaryu8_ {
() => {
// Module: crate::binary
// Provides: {"u8_"}
// Dependencies: {}
fn u8_ < Input , Error , const PARTIAL : bool > (input : & mut Input) -> Result < u8 , Error > where Input : StreamIsPartial + Stream < Token = u8 > , Error : ParserError < Input > , { input . next_token () . ok_or_else (| | { if PARTIAL && input . is_partial () { ParserError :: incomplete (input , Needed :: new (1)) } else { ParserError :: from_input (input) } }) }
};
}
