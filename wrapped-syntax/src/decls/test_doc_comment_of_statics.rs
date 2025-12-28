macro_rules! test_doc_comment_of_statics {
    () => {
        # [test] fn test_doc_comment_of_statics () { let file = SourceFile :: parse (r#"
        /// Number of levels
        static LEVELS: i32 = 0;
        "# , parser :: Edition :: CURRENT ,) . ok () . unwrap () ; let st = file . syntax () . descendants () . find_map (Static :: cast) . unwrap () ; assert_eq ! (" Number of levels" , st . doc_comments () . doc_comment_text () . unwrap ()) ; }
    };
}

test_doc_comment_of_statics!();