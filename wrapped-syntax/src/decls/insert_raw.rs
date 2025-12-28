macro_rules! deps {
    () => {
        Position!();
        Element!();
    };
}

macro_rules! insert_raw {
    () => {
        deps!();
        pub fn insert_raw (position : Position , elem : impl Element) { insert_all_raw (position , vec ! [elem . syntax_element ()]) ; }
    };
}

insert_raw!();