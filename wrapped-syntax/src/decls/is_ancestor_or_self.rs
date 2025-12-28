macro_rules! deps {
    () => {
        SyntaxNode!();
    };
}

macro_rules! is_ancestor_or_self {
    () => {
        deps!();
        fn is_ancestor_or_self (node : & SyntaxNode , ancestor : & SyntaxNode) -> bool { node == ancestor || node . ancestors () . any (| it | & it == ancestor) }
    };
}

is_ancestor_or_self!()