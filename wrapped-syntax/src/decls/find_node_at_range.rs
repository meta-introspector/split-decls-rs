macro_rules! deps {
    () => {
        SyntaxNode!();
        AstNode!();
    };
}

macro_rules! find_node_at_range {
    () => {
        deps!();
        pub fn find_node_at_range < N : AstNode > (syntax : & SyntaxNode , range : TextRange) -> Option < N > { syntax . covering_element (range) . ancestors () . find_map (N :: cast) }
    };
}

find_node_at_range!()