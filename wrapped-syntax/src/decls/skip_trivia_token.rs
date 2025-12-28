macro_rules! deps {
    () => {
        SyntaxToken!();
    };
}

macro_rules! skip_trivia_token {
    () => {
        deps!();
        # [doc = " Skip to next non `trivia` token"] pub fn skip_trivia_token (mut token : SyntaxToken , direction : Direction) -> Option < SyntaxToken > { while token . kind () . is_trivia () { token = match direction { Direction :: Next => token . next_token () ? , Direction :: Prev => token . prev_token () ? , } } Some (token) }
    };
}

skip_trivia_token!()