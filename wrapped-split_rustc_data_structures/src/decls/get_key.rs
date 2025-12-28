macro_rules! deps {
    () => {
        Element!();
    };
}

macro_rules! get_key {
    () => {
        deps!();
        fn get_key (data : & Element) -> usize { data . 0 }
    };
}

get_key!();