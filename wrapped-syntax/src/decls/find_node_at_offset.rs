macro_rules! deps {
    () => {
        SyntaxNode!();
        AstNode!();
    };
}

macro_rules! find_node_at_offset {
    () => {
        deps!();
        # [doc = " Finds a node of specific Ast type at offset. Note that this is slightly"] # [doc = " imprecise: if the cursor is strictly between two nodes of the desired type,"] # [doc = " as in"] # [doc = ""] # [doc = " ```ignore"] # [doc = " struct Foo {}|struct Bar;"] # [doc = " ```"] # [doc = ""] # [doc = " then the shorter node will be silently preferred."] pub fn find_node_at_offset < N : AstNode > (syntax : & SyntaxNode , offset : TextSize) -> Option < N > { ancestors_at_offset (syntax , offset) . find_map (N :: cast) }
    };
}

find_node_at_offset!();