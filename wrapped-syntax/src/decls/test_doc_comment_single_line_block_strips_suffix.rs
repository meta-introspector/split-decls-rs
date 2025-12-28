macro_rules! test_doc_comment_single_line_block_strips_suffix {
    () => {
        # [test] fn test_doc_comment_single_line_block_strips_suffix () { let file = SourceFile :: parse (r#"
        /** this is mod foo*/
        mod foo {}
        "# , parser :: Edition :: CURRENT ,) . ok () . unwrap () ; let module = file . syntax () . descendants () . find_map (Module :: cast) . unwrap () ; assert_eq ! (" this is mod foo" , module . doc_comments () . doc_comment_text () . unwrap ()) ; }
    };
}

test_doc_comment_single_line_block_strips_suffix!()