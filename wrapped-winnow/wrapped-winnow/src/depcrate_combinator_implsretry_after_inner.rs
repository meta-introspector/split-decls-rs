// Generated macro for retry_after_inner (function)
macro_rules! Depcrate_combinator_implsretry_after_inner {
() => {
// Module: crate::combinator::impls
// Provides: {"retry_after_inner"}
// Dependencies: {}
# [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] fn retry_after_inner < P , R , I , O , E > (parser : & mut P , recover : & mut R , i : & mut I) -> Result < O , E > where P : Parser < I , O , E > , R : Parser < I , () , E > , I : Stream , I : Recover < E > , E : ParserError < I > + FromRecoverableError < I , E > , { loop { let token_start = i . checkpoint () ; let mut err = match parser . parse_next (i) { Ok (o) => { return Ok (o) ; } Err (e) if e . is_incomplete () => return Err (e) , Err (err) => err , } ; let err_start = i . checkpoint () ; let err_start_eof_offset = i . eof_offset () ; if recover . parse_next (i) . is_ok () { let i_eof_offset = i . eof_offset () ; if err_start_eof_offset == i_eof_offset { } else if let Err (err_) = i . record_err (& token_start , & err_start , err) { err = err_ ; } else { continue ; } } i . reset (& err_start) ; err = E :: from_recoverable_error (& token_start , & err_start , i , err) ; return Err (err) ; } }
};
}
