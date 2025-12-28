macro_rules! deps {
    () => {
        Element!();
        SyntaxElement!();
    };
}

macro_rules! replace_with_many {
    () => {
        deps!();
        pub fn replace_with_many (old : impl Element , new : Vec < SyntaxElement >) { let old = old . syntax_element () ; replace_all (old . clone () ..= old , new) ; }
    };
}

replace_with_many!()