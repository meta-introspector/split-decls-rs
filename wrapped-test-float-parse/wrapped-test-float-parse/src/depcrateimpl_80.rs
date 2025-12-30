// Generated macro for impl_80 (impl)
macro_rules! Depcrateimpl_80 {
() => {
// Module: crate
// Provides: {"impl_80"}
// Dependencies: {}
impl fmt :: Display for CheckFailure { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { CheckFailure :: UnexpectedZero => { write ! (f , "incorrectly rounded to 0 (expected nonzero)") } CheckFailure :: UnexpectedInf => { write ! (f , "incorrectly rounded to +inf (expected finite)") } CheckFailure :: UnexpectedNegInf => { write ! (f , "incorrectly rounded to -inf (expected finite)") } CheckFailure :: UnexpectedNan => write ! (f , "got a NaN where none was expected") , CheckFailure :: ExpectedNan => write ! (f , "expected a NaN but did not get it") , CheckFailure :: ExpectedInf => write ! (f , "expected +inf but did not get it") , CheckFailure :: ExpectedNegInf => write ! (f , "expected -inf but did not get it") , CheckFailure :: InvalidReal { error_float , error_str , incorrect_midpoint_rounding } => { if * incorrect_midpoint_rounding { write ! (f , "midpoint between two representable values did not correctly \
                        round to even; error: {error_str}") ? ; } else { write ! (f , "real number did not parse correctly; error: {error_str}") ? ; } if let Some (float) = error_float { write ! (f , " ({float})") ? ; } Ok (()) } CheckFailure :: ParsingFailed (e) => write ! (f , "parsing failed: {e}") , CheckFailure :: Panic (e) => write ! (f , "function panicked: {e}") , } } }
};
}
