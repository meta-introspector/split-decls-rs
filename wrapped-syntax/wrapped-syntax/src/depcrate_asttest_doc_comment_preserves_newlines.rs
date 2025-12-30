// Generated macro for test_doc_comment_preserves_newlines (function)
macro_rules! Depcrate_asttest_doc_comment_preserves_newlines {
() => {
// Module: crate::ast
// Provides: {"test_doc_comment_preserves_newlines"}
// Dependencies: {}
# [test] fn test_doc_comment_preserves_newlines () { let file = SourceFile :: parse (r#"
        /// this
        /// is
        /// mod
        /// foo
        mod foo {}
        "# , parser :: Edition :: CURRENT ,) . ok () . unwrap () ; let module = file . syntax () . descendants () . find_map (Module :: cast) . unwrap () ; assert_eq ! (" this\n is\n mod\n foo" , module . doc_comments () . doc_comment_text () . unwrap ()) ; }
};
}
