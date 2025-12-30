// Generated macro for test_doc_comment_none (function)
macro_rules! Depcrate_asttest_doc_comment_none {
() => {
// Module: crate::ast
// Provides: {"test_doc_comment_none"}
// Dependencies: {}
# [test] fn test_doc_comment_none () { let file = SourceFile :: parse (r#"
        // non-doc
        mod foo {}
        "# , parser :: Edition :: CURRENT ,) . ok () . unwrap () ; let module = file . syntax () . descendants () . find_map (Module :: cast) . unwrap () ; assert ! (module . doc_comments () . doc_comment_text () . is_none ()) ; }
};
}
