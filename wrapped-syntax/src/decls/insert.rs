macro_rules! deps {
    () => {
        Element!();
        Position!();
    };
}

macro_rules! insert {
    () => {
        deps!();
        pub fn insert (position : Position , elem : impl Element) { insert_all (position , vec ! [elem . syntax_element ()]) ; }
    };
}

insert!()