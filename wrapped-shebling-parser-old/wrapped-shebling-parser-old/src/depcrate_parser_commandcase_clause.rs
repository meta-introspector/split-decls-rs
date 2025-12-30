// Generated macro for case_clause (function)
macro_rules! Depcrate_parser_commandcase_clause {
() => {
// Module: crate::parser::command
// Provides: {"case_clause"}
// Dependencies: {}
fn case_clause (span : Span) -> ParseResult < CaseClause > { fn pattern_word (span : Span) -> ParseResult < Word > { let (span , (word , range)) = ranged (map (many1 (word_sgmt) , Word :: new)) (span) ? ; let esac = Keyword :: Esac . token () ; if word . as_lit () . is_some_and (| lit | lit == esac) { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: SusToken) . label ("literal 'esac'" , range) . help ("If intended, quote it. Else add a semicolon or new line before it.") ,) ; } Ok ((span , word)) } fn sep (span : Span) -> ParseResult < Option < ClauseSep > > { alt ((map (alt ((token (ClauseSep :: Continue) , token (ClauseSep :: Fallthrough) , token (ClauseSep :: Break) ,)) , Some ,) , map (peek (pair (linebreak , token (Keyword :: Esac))) , | _ | None) ,)) (span) } let (span , _) = not (peek (token (Keyword :: Esac))) (span) ? ; let (span , _) = pair (opt (char ('(')) , trivia) (span) ? ; let (span , pattern) = separated_list1 (terminated (char ('|') , trivia) , terminated (pattern_word , trivia) ,) (span) ? ; let (span , _) = pair (cut (context ("expected a closing ')'!" , char (')'))) , linebreak ,) (span) ? ; let (span , cmd) = alt ((map (peek (sep) , | _ | None) , map (term , Some))) (span) ? ; let (span , sep) = delimited (cut (context ("did you forget the separator in the previous clause?" , not (char (')')) ,)) , sep , pair (trivia , linebreak) ,) (span) ? ; Ok ((span , CaseClause :: new (pattern , cmd , sep))) }
};
}
