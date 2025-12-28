macro_rules! deps {
    () => {
        SyntaxNode!();
    };
}

macro_rules! ancestors_at_offset {
    () => {
        deps!();
        # [doc = " Returns ancestors of the node at the offset, sorted by length. This should"] # [doc = " do the right thing at an edge, e.g. when searching for expressions at `{"] # [doc = " $0foo }` we will get the name reference instead of the whole block, which"] # [doc = " we would get if we just did `find_token_at_offset(...).flat_map(|t|"] # [doc = " t.parent().ancestors())`."] pub fn ancestors_at_offset (node : & SyntaxNode , offset : TextSize ,) -> impl Iterator < Item = SyntaxNode > { node . token_at_offset (offset) . map (| token | token . parent_ancestors ()) . kmerge_by (| node1 , node2 | node1 . text_range () . len () < node2 . text_range () . len ()) }
    };
}

ancestors_at_offset!();