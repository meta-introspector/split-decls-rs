// Generated macro for word (function)
macro_rules! Depcrate_parser_wordword {
() => {
// Module: crate::parser::word
// Provides: {"word"}
// Dependencies: {}
pub (super) fn word (span : Span) -> ParseResult < Word > { let (span , (word , range)) = ranged (map (context ("invalid or missing word!" , many1 (word_sgmt)) , Word :: new ,)) (span) ? ; if let Some (lit) = word . as_lit () { if vec ! [Keyword :: Do , Keyword :: Done , Keyword :: Esac , Keyword :: Fi , Keyword :: Then ,] . into_iter () . any (| keyword | keyword . token () == lit) { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: SusToken) . label (format ! ("literal '{}'" , lit) , range) . help ("If intended, quote it. Else add a semicolon or new line before it.") ,) ; } } Ok ((span , word)) }
};
}
