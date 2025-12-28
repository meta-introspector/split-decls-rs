macro_rules! test_doc_comment_preserves_indents {
    () => {
        # [test] fn test_doc_comment_preserves_indents () { let file = SourceFile :: parse (r#"
        /// doc1
        /// ```
        /// fn foo() {
        ///     // ...
        /// }
        /// ```
        mod foo {}
        "# , parser :: Edition :: CURRENT ,) . ok () . unwrap () ; let module = file . syntax () . descendants () . find_map (Module :: cast) . unwrap () ; assert_eq ! (" doc1\n ```\n fn foo() {\n     // ...\n }\n ```" , module . doc_comments () . doc_comment_text () . unwrap ()) ; }
    };
}

test_doc_comment_preserves_indents!()