macro_rules! deps {
    () => {
        Element!();
    };
}

macro_rules! replace {
    () => {
        deps!();
        pub fn replace (old : impl Element , new : impl Element) { replace_with_many (old , vec ! [new . syntax_element ()]) ; }
    };
}

replace!()