// Generated macro for validate (function)
macro_rules! Depcrate_validatevalidate {
() => {
// Module: crate::validate
// Provides: {"validate"}
// Dependencies: {}
# [doc = " Validate that a string parses correctly"] pub fn validate < F : Float > (input : & str) -> Result < () , CheckError > { let parsed = std :: panic :: catch_unwind (| | { input . parse :: < F > () . map_err (| e | CheckError { fail : CheckFailure :: ParsingFailed (e . to_string () . into ()) , input : input . into () , float_res : "none" . into () , }) }) . map_err (| e | convert_panic_error (& e , input)) ? ? ; let decoded = decode (parsed) ; let rational = Rational :: parse (input) ; decoded . check (rational , input) }
};
}
