macro_rules! test_doc_comment_none {
    () => {
        # [test] fn test_doc_comment_none () { let file = SourceFile :: parse (r#"
        // non-doc
        mod foo {}
        "# , parser :: Edition :: CURRENT ,) . ok () . unwrap () ; let module = file . syntax () . descendants () . find_map (Module :: cast) . unwrap () ; assert ! (module . doc_comments () . doc_comment_text () . is_none ()) ; }
    };
}

test_doc_comment_none!();