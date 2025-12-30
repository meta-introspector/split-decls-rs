// Generated macro for take_exp (function)
macro_rules! Depcrate_asciitake_exp {
() => {
// Module: crate::ascii
// Provides: {"take_exp"}
// Dependencies: {}
# [allow (clippy :: trait_duplication_in_bounds)] fn take_exp < I , E : ParserError < I > > (input : & mut I) -> Result < () , E > where I : StreamIsPartial , I : Stream , I : Compare < char > , < I as Stream > :: Token : AsChar + Clone , < I as Stream > :: IterOffsets : Clone , I : AsBStr , { dispatch ! { opt (peek (any) . map (AsChar :: as_char)) ; Some ('E') | Some ('e') => (one_of (['e' , 'E']) , opt (one_of (['+' , '-'])) , digit1) . void () , _ => empty , } . parse_next (input) }
};
}
