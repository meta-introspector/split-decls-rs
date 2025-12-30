// Generated macro for skip_trivia_token (function)
macro_rules! Depcrate_algoskip_trivia_token {
() => {
// Module: crate::algo
// Provides: {"skip_trivia_token"}
// Dependencies: {}
# [doc = " Skip to next non `trivia` token"] pub fn skip_trivia_token (mut token : SyntaxToken , direction : Direction) -> Option < SyntaxToken > { while token . kind () . is_trivia () { token = match direction { Direction :: Next => token . next_token () ? , Direction :: Prev => token . prev_token () ? , } } Some (token) }
};
}
