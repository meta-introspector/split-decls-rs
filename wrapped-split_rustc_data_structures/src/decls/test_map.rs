macro_rules! deps {
    () => {
        Element!();
    };
}

macro_rules! test_map {
    () => {
        deps!();
        fn test_map () -> Vec < Element > { let mut data = vec ! [(3 , "three-a") , (0 , "zero") , (3 , "three-b") , (22 , "twenty-two")] ; data . sort_by_key (get_key) ; data }
    };
}

test_map!();