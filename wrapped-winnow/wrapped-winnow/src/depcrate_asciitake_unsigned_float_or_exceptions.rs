// Generated macro for take_unsigned_float_or_exceptions (function)
macro_rules! Depcrate_asciitake_unsigned_float_or_exceptions {
() => {
// Module: crate::ascii
// Provides: {"take_unsigned_float_or_exceptions"}
// Dependencies: {}
# [allow (clippy :: trait_duplication_in_bounds)] fn take_unsigned_float_or_exceptions < I , E : ParserError < I > > (input : & mut I) -> Result < () , E > where I : StreamIsPartial , I : Stream , I : Compare < Caseless < & 'static str > > , I : Compare < char > , < I as Stream > :: Token : AsChar + Clone , < I as Stream > :: IterOffsets : Clone , I : AsBStr , { dispatch ! { opt (peek (any) . map (AsChar :: as_char)) ; Some ('I') | Some ('i') => (Caseless ("inf") , opt (Caseless ("inity"))) . void () , Some ('.') => ('.' , digit1 , take_exp) . void () , _ => (digit1 , opt (('.' , opt (digit1))) , take_exp) . void () , } . parse_next (input) }
};
}
