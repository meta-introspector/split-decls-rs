macro_rules! deps {
    () => {
        TestCache!();
    };
}

macro_rules! empty {
    () => {
        deps!();
        # [test] fn empty () { let mut cache = TestCache :: new () ; assert_eq ! (cache . is_empty () , true) ; assert_eq ! (items (& mut cache) , []) ; cache . insert (1) ; assert_eq ! (cache . is_empty () , false) ; }
    };
}

empty!();