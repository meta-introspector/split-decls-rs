macro_rules! is_test_attribute {
    () => {
        fn is_test_attribute (attr : & Attribute) -> bool { let path = match & attr . meta { syn :: Meta :: Path (path) => path , _ => return false , } ; let candidates = [["core" , "prelude" , "*" , "test"] , ["std" , "prelude" , "*" , "test"] ,] ; if path . leading_colon . is_none () && path . segments . len () == 1 && path . segments [0] . arguments . is_none () && path . segments [0] . ident == "test" { return true ; } else if path . segments . len () != candidates [0] . len () { return false ; } candidates . into_iter () . any (| segments | { path . segments . iter () . zip (segments) . all (| (segment , path) | { segment . arguments . is_none () && (path == "*" || segment . ident == path) }) }) }
    };
}

is_test_attribute!();