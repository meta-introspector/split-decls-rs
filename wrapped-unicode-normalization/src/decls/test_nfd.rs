macro_rules! test_nfd {
    () => {
        # [test] fn test_nfd () { macro_rules ! t { ($ input : expr , $ expected : expr) => { assert_eq ! ($ input . nfd () . to_string () , $ expected) ; assert_eq ! ($ input . chars () . map (| c | c) . nfd () . collect ::< String > () , $ expected) ; } ; } t ! ("abc" , "abc") ; t ! ("\u{1e0b}\u{1c4}" , "d\u{307}\u{1c4}") ; t ! ("\u{2026}" , "\u{2026}") ; t ! ("\u{2126}" , "\u{3a9}") ; t ! ("\u{1e0b}\u{323}" , "d\u{323}\u{307}") ; t ! ("\u{1e0d}\u{307}" , "d\u{323}\u{307}") ; t ! ("a\u{301}" , "a\u{301}") ; t ! ("\u{301}a" , "\u{301}a") ; t ! ("\u{d4db}" , "\u{1111}\u{1171}\u{11b6}") ; t ! ("\u{ac1c}" , "\u{1100}\u{1162}") ; }
    };
}

test_nfd!()