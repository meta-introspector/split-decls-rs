macro_rules! deps {
    () => {
        SyntaxElement!();
        SyntaxToken!();
    };
}

macro_rules! ws_between {
    () => {
        deps!();
        fn ws_between (left : & SyntaxElement , right : & SyntaxElement) -> Option < SyntaxToken > { if left . kind () == SyntaxKind :: WHITESPACE || right . kind () == SyntaxKind :: WHITESPACE { return None ; } if right . kind () == T ! [;] || right . kind () == T ! [,] { return None ; } if left . kind () == T ! [<] || right . kind () == T ! [>] { return None ; } if left . kind () == T ! [&] && right . kind () == SyntaxKind :: LIFETIME { return None ; } if right . kind () == SyntaxKind :: GENERIC_ARG_LIST { return None ; } if right . kind () == SyntaxKind :: USE { let mut indent = IndentLevel :: from_element (left) ; if left . kind () == SyntaxKind :: USE { indent . 0 = IndentLevel :: from_element (right) . 0 . max (indent . 0) ; } return Some (make :: tokens :: whitespace (& format ! ("\n{indent}"))) ; } if left . kind () == SyntaxKind :: ATTR { let mut indent = IndentLevel :: from_element (right) ; if right . kind () == SyntaxKind :: ATTR { indent . 0 = IndentLevel :: from_element (left) . 0 . max (indent . 0) ; } return Some (make :: tokens :: whitespace (& format ! ("\n{indent}"))) ; } Some (make :: tokens :: single_space ()) }
    };
}

ws_between!()