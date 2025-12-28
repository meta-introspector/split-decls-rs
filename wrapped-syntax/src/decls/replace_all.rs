macro_rules! deps {
    () => {
        SyntaxElement!();
    };
}

macro_rules! replace_all {
    () => {
        deps!();
        pub fn replace_all (range : RangeInclusive < SyntaxElement > , new : Vec < SyntaxElement >) { let start = range . start () . index () ; let end = range . end () . index () ; let parent = range . start () . parent () . unwrap () ; parent . splice_children (start .. end + 1 , new) ; }
    };
}

replace_all!();