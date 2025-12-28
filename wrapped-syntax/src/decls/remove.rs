macro_rules! deps {
    () => {
        Element!();
    };
}

macro_rules! remove {
    () => {
        deps!();
        pub fn remove (elem : impl Element) { elem . syntax_element () . detach () ; }
    };
}

remove!()