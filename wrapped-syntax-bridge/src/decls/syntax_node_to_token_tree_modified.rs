macro_rules! deps {
    () => {
        SpanMapper!();
        DocCommentDesugarMode!();
        Converter!();
    };
}

macro_rules! syntax_node_to_token_tree_modified {
    () => {
        deps!();
        # [doc = " Converts a syntax tree to a [`tt::Subtree`] using the provided span map to populate the"] # [doc = " subtree's spans. Additionally using the append and remove parameters, the additional tokens can"] # [doc = " be injected or hidden from the output."] pub fn syntax_node_to_token_tree_modified < Ctx , SpanMap > (node : & SyntaxNode , map : SpanMap , append : FxHashMap < SyntaxElement , Vec < tt :: Leaf < SpanData < Ctx > > > > , remove : FxHashSet < SyntaxElement > , call_site : SpanData < Ctx > , mode : DocCommentDesugarMode ,) -> tt :: TopSubtree < SpanData < Ctx > > where SpanMap : SpanMapper < SpanData < Ctx > > , SpanData < Ctx > : Copy + fmt :: Debug , { let mut c = Converter :: new (node , map , append , remove , call_site , mode) ; convert_tokens (& mut c) }
    };
}

syntax_node_to_token_tree_modified!()