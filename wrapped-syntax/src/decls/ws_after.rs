macro_rules! deps {
    () => {
        SyntaxElement!();
        Position!();
        SyntaxToken!();
        PositionRepr!();
    };
}

macro_rules! ws_after {
    () => {
        deps!();
        fn ws_after (position : & Position , new : & SyntaxElement) -> Option < SyntaxToken > { let next = match & position . repr { PositionRepr :: FirstChild (parent) => parent . first_child_or_token () ? , PositionRepr :: After (sibling) => sibling . next_sibling_or_token () ? , } ; ws_between (new , & next) }
    };
}

ws_after!()