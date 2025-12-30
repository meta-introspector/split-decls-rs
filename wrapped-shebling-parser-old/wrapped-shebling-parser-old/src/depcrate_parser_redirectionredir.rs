// Generated macro for redir (function)
macro_rules! Depcrate_parser_redirectionredir {
() => {
// Module: crate::parser::redirection
// Provides: {"redir"}
// Dependencies: {}
pub (super) fn redir (span : Span) -> ParseResult < Redir > { let (span , desc) = context ("invalid file descriptor!" , alt ((map (alt ((map (char ('&') , | _ | FileDesc :: StdOutErr) , into (delimited (char ('{') , identifier , char ('}'))) , map (digit1 , | _ | FileDesc :: Number) ,)) , Some ,) , map (peek (one_of ("<>")) , | _ | None) ,)) ,) (span) ? ; let (span , (op , word)) = preceded (context ("expected a redirection operator!" , peek (one_of ("<>"))) , alt ((separated_pair (token (RedirOp :: TLess) , trivia , word) , separated_pair (alt ((token (RedirOp :: GreatAnd) , token (RedirOp :: LessAnd))) , trivia , map (alt ((delimited (char ('{') , identifier , char ('}')) , recognize_string (pair (digit1 , opt (char ('-')))) , recognize_string (char ('-')) ,)) , | word | Word :: new (vec ! [WordSgmt :: Lit (word)]) ,) ,) , separated_pair (alt ((token (RedirOp :: DGreat) , token (RedirOp :: LessGreat) , token (RedirOp :: GreatAnd) , token (RedirOp :: LessAnd) , token (RedirOp :: Clobber) , token (RedirOp :: DLess) , token (RedirOp :: Less) , token (RedirOp :: Great) ,)) , trivia , word ,) ,)) ,) (span) ? ; let (span , _) = trivia (span) ? ; Ok ((span , Redir :: new (desc , op , word))) }
};
}
