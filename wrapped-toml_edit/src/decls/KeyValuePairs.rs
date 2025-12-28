macro_rules! deps {
    () => {
        Item!();
        Key!();
    };
}

macro_rules! KeyValuePairs {
    () => {
        deps!();
        pub (crate) type KeyValuePairs = IndexMap < Key , Item > ;
    };
}

KeyValuePairs!();