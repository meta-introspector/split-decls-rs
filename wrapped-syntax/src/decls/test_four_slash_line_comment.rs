macro_rules! test_four_slash_line_comment {
    () => {
        # [test] fn test_four_slash_line_comment () { let file = SourceFile :: parse (r#"
        //// too many slashes to be a doc comment
        /// doc comment
        mod foo {}
        "# , parser :: Edition :: CURRENT ,) . ok () . unwrap () ; let module = file . syntax () . descendants () . find_map (Module :: cast) . unwrap () ; assert_eq ! (" doc comment" , module . doc_comments () . doc_comment_text () . unwrap ()) ; }
    };
}

test_four_slash_line_comment!();