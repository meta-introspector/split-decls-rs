macro_rules! test_comments_preserve_trailing_whitespace {
    () => {
        # [test] fn test_comments_preserve_trailing_whitespace () { let file = SourceFile :: parse ("\n/// Representation of a Realm.   \n/// In the specification these are called Realm Records.\nstruct Realm {}" , parser :: Edition :: CURRENT ,) . ok () . unwrap () ; let def = file . syntax () . descendants () . find_map (Struct :: cast) . unwrap () ; assert_eq ! (" Representation of a Realm.   \n In the specification these are called Realm Records." , def . doc_comments () . doc_comment_text () . unwrap ()) ; }
    };
}

test_comments_preserve_trailing_whitespace!()