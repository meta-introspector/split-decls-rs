macro_rules! test_nfkd {
    () => {
        # [test] fn test_nfkd () { macro_rules ! t { ($ input : expr , $ expected : expr) => { assert_eq ! ($ input . nfkd () . to_string () , $ expected) ; } ; } t ! ("abc" , "abc") ; t ! ("\u{1e0b}\u{1c4}" , "d\u{307}DZ\u{30c}") ; t ! ("\u{2026}" , "...") ; t ! ("\u{2126}" , "\u{3a9}") ; t ! ("\u{1e0b}\u{323}" , "d\u{323}\u{307}") ; t ! ("\u{1e0d}\u{307}" , "d\u{323}\u{307}") ; t ! ("a\u{301}" , "a\u{301}") ; t ! ("\u{301}a" , "\u{301}a") ; t ! ("\u{d4db}" , "\u{1111}\u{1171}\u{11b6}") ; t ! ("\u{ac1c}" , "\u{1100}\u{1162}") ; }
    };
}

test_nfkd!()