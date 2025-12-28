macro_rules! test_nfkc {
    () => {
        # [test] fn test_nfkc () { macro_rules ! t { ($ input : expr , $ expected : expr) => { assert_eq ! ($ input . nfkc () . to_string () , $ expected) ; } ; } t ! ("abc" , "abc") ; t ! ("\u{1e0b}\u{1c4}" , "\u{1e0b}D\u{17d}") ; t ! ("\u{2026}" , "...") ; t ! ("\u{2126}" , "\u{3a9}") ; t ! ("\u{1e0b}\u{323}" , "\u{1e0d}\u{307}") ; t ! ("\u{1e0d}\u{307}" , "\u{1e0d}\u{307}") ; t ! ("a\u{301}" , "\u{e1}") ; t ! ("\u{301}a" , "\u{301}a") ; t ! ("\u{d4db}" , "\u{d4db}") ; t ! ("\u{ac1c}" , "\u{ac1c}") ; t ! ("a\u{300}\u{305}\u{315}\u{5ae}b" , "\u{e0}\u{5ae}\u{305}\u{315}b") ; }
    };
}

test_nfkc!()