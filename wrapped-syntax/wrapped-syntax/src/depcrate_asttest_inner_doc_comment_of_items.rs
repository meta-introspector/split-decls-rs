// Generated macro for test_inner_doc_comment_of_items (function)
macro_rules! Depcrate_asttest_inner_doc_comment_of_items {
() => {
// Module: crate::ast
// Provides: {"test_inner_doc_comment_of_items"}
// Dependencies: {}
# [test] fn test_inner_doc_comment_of_items () { let file = SourceFile :: parse (r#"
        //! doc
        // non-doc
        mod foo {}
        "# , parser :: Edition :: CURRENT ,) . ok () . unwrap () ; let module = file . syntax () . descendants () . find_map (Module :: cast) . unwrap () ; assert ! (module . doc_comments () . doc_comment_text () . is_none ()) ; }
};
}
