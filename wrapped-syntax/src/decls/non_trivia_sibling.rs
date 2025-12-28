macro_rules! deps {
    () => {
        SyntaxElement!();
    };
}

macro_rules! non_trivia_sibling {
    () => {
        deps!();
        # [doc = " Finds the first sibling in the given direction which is not `trivia`"] pub fn non_trivia_sibling (element : SyntaxElement , direction : Direction) -> Option < SyntaxElement > { return match element { NodeOrToken :: Node (node) => node . siblings_with_tokens (direction) . skip (1) . find (not_trivia) , NodeOrToken :: Token (token) => token . siblings_with_tokens (direction) . skip (1) . find (not_trivia) , } ; fn not_trivia (element : & SyntaxElement) -> bool { match element { NodeOrToken :: Node (_) => true , NodeOrToken :: Token (token) => ! token . kind () . is_trivia () , } } }
    };
}

non_trivia_sibling!();