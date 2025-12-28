macro_rules! deps {
    () => {
        SyntaxElement!();
    };
}

macro_rules! remove_all {
    () => {
        deps!();
        pub fn remove_all (range : RangeInclusive < SyntaxElement >) { replace_all (range , Vec :: new ()) ; }
    };
}

remove_all!()