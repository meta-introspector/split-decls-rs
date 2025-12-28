macro_rules! deps {
    () => {
        TtTreeSink!();
    };
}

macro_rules! token_tree_to_syntax_node {
    () => {
        deps!();
        # [doc = " Converts a [`tt::Subtree`] back to a [`SyntaxNode`]."] # [doc = " The produced `SpanMap` contains a mapping from the syntax nodes offsets to the subtree's spans."] pub fn token_tree_to_syntax_node < Ctx > (tt : & tt :: TopSubtree < SpanData < Ctx > > , entry_point : parser :: TopEntryPoint , span_to_edition : & mut dyn FnMut (Ctx) -> Edition , top_edition : Edition ,) -> (Parse < SyntaxNode > , SpanMap < Ctx >) where Ctx : Copy + fmt :: Debug + PartialEq + PartialEq + Eq + Hash , { let buffer = tt . view () . strip_invisible () ; let parser_input = to_parser_input (buffer , span_to_edition) ; let parser_output = entry_point . parse (& parser_input , top_edition) ; let mut tree_sink = TtTreeSink :: new (buffer . cursor ()) ; for event in parser_output . iter () { match event { parser :: Step :: Token { kind , n_input_tokens : n_raw_tokens } => { tree_sink . token (kind , n_raw_tokens) } parser :: Step :: FloatSplit { ends_in_dot : has_pseudo_dot } => { tree_sink . float_split (has_pseudo_dot) } parser :: Step :: Enter { kind } => tree_sink . start_node (kind) , parser :: Step :: Exit => tree_sink . finish_node () , parser :: Step :: Error { msg } => tree_sink . error (msg . to_owned ()) , } } tree_sink . finish () }
    };
}

token_tree_to_syntax_node!();