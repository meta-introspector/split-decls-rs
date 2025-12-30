// Generated macro for check_punct_spacing (function)
macro_rules! Depcrate_testscheck_punct_spacing {
() => {
// Module: crate::tests
// Provides: {"check_punct_spacing"}
// Dependencies: {}
fn check_punct_spacing (fixture : & str) { let source_file = ast :: SourceFile :: parse (fixture , span :: Edition :: CURRENT) . ok () . unwrap () ; let subtree = syntax_node_to_token_tree (source_file . syntax () , DummyTestSpanMap , DUMMY , DocCommentDesugarMode :: Mbe ,) ; let mut annotations : FxHashMap < _ , _ > = extract_annotations (fixture) . into_iter () . map (| (range , annotation) | { let spacing = match annotation . as_str () { "Alone" => Spacing :: Alone , "Joint" => Spacing :: Joint , a => panic ! ("unknown annotation: {a}") , } ; (range , spacing) }) . collect () ; let mut cursor = Cursor :: new (& subtree . 0) ; while ! cursor . eof () { while let Some (token_tree) = cursor . token_tree () { if let tt :: TokenTree :: Leaf (Leaf :: Punct (Punct { spacing , span : Span { range , .. } , .. })) = token_tree && let Some (expected) = annotations . remove (range) { assert_eq ! (expected , * spacing) ; } cursor . bump () ; } cursor . bump_or_end () ; } assert ! (annotations . is_empty () , "unchecked annotations: {annotations:?}") ; }
};
}
