// Generated macro for till_line_ending_ (function)
macro_rules! Depcrate_asciitill_line_ending_ {
() => {
// Module: crate::ascii
// Provides: {"till_line_ending_"}
// Dependencies: {}
fn till_line_ending_ < I , E : ParserError < I > , const PARTIAL : bool > (input : & mut I ,) -> Result < < I as Stream > :: Slice , E > where I : StreamIsPartial , I : Stream , I : Compare < & 'static str > , I : FindSlice < (char , char) > , < I as Stream > :: Token : AsChar + Clone , { let res = match take_until (0 .. , ('\r' , '\n')) . parse_next (input) . map_err (| e : E | e) { Ok (slice) => slice , Err (err) if err . is_backtrack () => input . finish () , Err (err) => { return Err (err) ; } } ; if matches ! (input . compare ("\r") , CompareResult :: Ok (_)) { let comp = input . compare ("\r\n") ; match comp { CompareResult :: Ok (_) => { } CompareResult :: Incomplete if PARTIAL && input . is_partial () => { return Err (ParserError :: incomplete (input , Needed :: Unknown)) ; } CompareResult :: Incomplete | CompareResult :: Error => { return Err (ParserError :: from_input (input)) ; } } } Ok (res) }
};
}
