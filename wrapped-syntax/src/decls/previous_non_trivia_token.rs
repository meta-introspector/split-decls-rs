macro_rules! deps {
    () => {
        SyntaxElement!();
        SyntaxToken!();
    };
}

macro_rules! previous_non_trivia_token {
    () => {
        deps!();
        pub fn previous_non_trivia_token (e : impl Into < SyntaxElement >) -> Option < SyntaxToken > { let mut token = match e . into () { SyntaxElement :: Node (n) => n . first_token () ? , SyntaxElement :: Token (t) => t , } . prev_token () ; while let Some (inner) = token { if ! inner . kind () . is_trivia () { return Some (inner) ; } else { token = inner . prev_token () ; } } None }
    };
}

previous_non_trivia_token!()