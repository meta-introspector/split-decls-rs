macro_rules! deps {
    () => {
        TestCache!();
    };
}

macro_rules! clear {
    () => {
        deps!();
        # [test] fn clear () { let mut cache = TestCache :: default () ; cache . insert (1) ; cache . clear () ; assert_eq ! (items (& mut cache) , [] , "all items evicted") ; cache . insert (1) ; cache . insert (2) ; cache . insert (3) ; cache . insert (4) ; assert_eq ! (items (& mut cache) , [4 , 3 , 2 , 1]) ; cache . clear () ; assert_eq ! (items (& mut cache) , [] , "all items evicted again") ; }
    };
}

clear!()