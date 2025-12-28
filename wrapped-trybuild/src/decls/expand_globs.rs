macro_rules! deps {
    () => {
        ExpandedTestSet!();
        ExpandedTest!();
        Test!();
    };
}

macro_rules! expand_globs {
    () => {
        deps!();
        pub (crate) fn expand_globs (tests : & [Test]) -> Vec < ExpandedTest > { let mut set = ExpandedTestSet :: new () ; for test in tests { match test . path . to_str () { Some (utf8) if utf8 . contains ('*') => match glob (utf8) { Ok (paths) => { let expected = test . expected ; for path in paths { set . insert (Test { path , expected } , None , true) ; } } Err (error) => set . insert (test . clone () , Some (error) , false) , } , _ => set . insert (test . clone () , None , false) , } } set . vec }
    };
}

expand_globs!();