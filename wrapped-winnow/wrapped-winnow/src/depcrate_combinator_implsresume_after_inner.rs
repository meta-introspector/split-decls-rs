// Generated macro for resume_after_inner (function)
macro_rules! Depcrate_combinator_implsresume_after_inner {
() => {
// Module: crate::combinator::impls
// Provides: {"resume_after_inner"}
// Dependencies: {}
# [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] fn resume_after_inner < P , R , I , O , E > (parser : & mut P , recover : & mut R , i : & mut I ,) -> Result < Option < O > , E > where P : Parser < I , O , E > , R : Parser < I , () , E > , I : Stream , I : Recover < E > , E : ParserError < I > + FromRecoverableError < I , E > , { let token_start = i . checkpoint () ; let mut err = match parser . parse_next (i) { Ok (o) => { return Ok (Some (o)) ; } Err (e) if e . is_incomplete () => return Err (e) , Err (err) => err , } ; let err_start = i . checkpoint () ; if recover . parse_next (i) . is_ok () { if let Err (err_) = i . record_err (& token_start , & err_start , err) { err = err_ ; } else { return Ok (None) ; } } i . reset (& err_start) ; err = FromRecoverableError :: from_recoverable_error (& token_start , & err_start , i , err) ; Err (err) }
};
}
