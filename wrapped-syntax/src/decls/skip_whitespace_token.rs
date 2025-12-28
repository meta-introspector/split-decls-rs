macro_rules! deps {
    () => {
        SyntaxToken!();
    };
}

macro_rules! skip_whitespace_token {
    () => {
        deps!();
        # [doc = " Skip to next non `whitespace` token"] pub fn skip_whitespace_token (mut token : SyntaxToken , direction : Direction) -> Option < SyntaxToken > { while token . kind () == SyntaxKind :: WHITESPACE { token = match direction { Direction :: Next => token . next_token () ? , Direction :: Prev => token . prev_token () ? , } } Some (token) }
    };
}

skip_whitespace_token!()