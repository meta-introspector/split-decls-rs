macro_rules! test_inner_doc_comment_of_items {
    () => {
        # [test] fn test_inner_doc_comment_of_items () { let file = SourceFile :: parse (r#"
        //! doc
        // non-doc
        mod foo {}
        "# , parser :: Edition :: CURRENT ,) . ok () . unwrap () ; let module = file . syntax () . descendants () . find_map (Module :: cast) . unwrap () ; assert ! (module . doc_comments () . doc_comment_text () . is_none ()) ; }
    };
}

test_inner_doc_comment_of_items!()