// Generated macro for take_float_or_exceptions (function)
macro_rules! Depcrate_asciitake_float_or_exceptions {
() => {
// Module: crate::ascii
// Provides: {"take_float_or_exceptions"}
// Dependencies: {}
# [allow (clippy :: trait_duplication_in_bounds)] fn take_float_or_exceptions < I , E : ParserError < I > > (input : & mut I) -> Result < < I as Stream > :: Slice , E > where I : StreamIsPartial , I : Stream , I : Compare < Caseless < & 'static str > > , I : Compare < char > , < I as Stream > :: Token : AsChar + Clone , < I as Stream > :: IterOffsets : Clone , I : AsBStr , { dispatch ! { opt (peek (any) . map (AsChar :: as_char)) ; Some ('N') | Some ('n') => Caseless ("nan") . void () , Some ('+') | Some ('-') => (any , take_unsigned_float_or_exceptions) . void () , _ => take_unsigned_float_or_exceptions , } . take () . parse_next (input) }
};
}
