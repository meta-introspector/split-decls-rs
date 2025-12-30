// Generated macro for test_doc_comment_multi_line_block_strips_suffix (function)
macro_rules! Depcrate_asttest_doc_comment_multi_line_block_strips_suffix {
() => {
// Module: crate::ast
// Provides: {"test_doc_comment_multi_line_block_strips_suffix"}
// Dependencies: {}
# [test] fn test_doc_comment_multi_line_block_strips_suffix () { let file = SourceFile :: parse (r#"
        /**
        this
        is
        mod foo
        */
        mod foo {}
        "# , parser :: Edition :: CURRENT ,) . ok () . unwrap () ; let module = file . syntax () . descendants () . find_map (Module :: cast) . unwrap () ; assert_eq ! ("\n        this\n        is\n        mod foo\n        " , module . doc_comments () . doc_comment_text () . unwrap ()) ; }
};
}
