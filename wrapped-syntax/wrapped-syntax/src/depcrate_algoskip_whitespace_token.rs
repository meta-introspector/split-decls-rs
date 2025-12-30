// Generated macro for skip_whitespace_token (function)
macro_rules! Depcrate_algoskip_whitespace_token {
() => {
// Module: crate::algo
// Provides: {"skip_whitespace_token"}
// Dependencies: {}
# [doc = " Skip to next non `whitespace` token"] pub fn skip_whitespace_token (mut token : SyntaxToken , direction : Direction) -> Option < SyntaxToken > { while token . kind () == SyntaxKind :: WHITESPACE { token = match direction { Direction :: Next => token . next_token () ? , Direction :: Prev => token . prev_token () ? , } } Some (token) }
};
}
