macro_rules! deps {
    () => {
        TestCache!();
    };
}

macro_rules! len {
    () => {
        deps!();
        # [test] fn len () { let mut cache = TestCache :: default () ; cache . insert (1) ; assert_eq ! (cache . len () , 1) ; assert_eq ! (items (& mut cache) , [1]) ; }
    };
}

len!()