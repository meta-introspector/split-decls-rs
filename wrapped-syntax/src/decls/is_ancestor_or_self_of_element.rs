macro_rules! deps {
    () => {
        SyntaxElement!();
        SyntaxNode!();
    };
}

macro_rules! is_ancestor_or_self_of_element {
    () => {
        deps!();
        fn is_ancestor_or_self_of_element (node : & SyntaxElement , ancestor : & SyntaxNode) -> bool { matches ! (node , SyntaxElement :: Node (node) if node == ancestor) || node . ancestors () . any (| it | & it == ancestor) }
    };
}

is_ancestor_or_self_of_element!()