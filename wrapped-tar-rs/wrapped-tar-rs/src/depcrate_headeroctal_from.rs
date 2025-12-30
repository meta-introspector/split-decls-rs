// Generated macro for octal_from (function)
macro_rules! Depcrate_headeroctal_from {
() => {
// Module: crate::header
// Provides: {"octal_from"}
// Dependencies: {}
fn octal_from (slice : & [u8]) -> io :: Result < u64 > { let trun = truncate (slice) ; let num = match str :: from_utf8 (trun) { Ok (n) => n , Err (_) => { return Err (other (& format ! ("numeric field did not have utf-8 text: {}" , String :: from_utf8_lossy (trun)))) ; } } ; match u64 :: from_str_radix (num . trim () , 8) { Ok (n) => Ok (n) , Err (_) => Err (other (& format ! ("numeric field was not a number: {}" , num))) , } }
};
}
