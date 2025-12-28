macro_rules! deps {
    () => {
        TestCache!();
    };
}

macro_rules! find {
    () => {
        deps!();
        # [test] fn find () { let mut cache = TestCache :: default () ; cache . insert (0) ; cache . insert (1) ; cache . insert (2) ; cache . insert (3) ; let result = cache . find (| x | * x == 5) . copied () ; assert_eq ! (result , None) ; assert_eq ! (items (& mut cache) , [3 , 2 , 1 , 0] , "Nothing is touched.") ; let result = cache . find (| x | * x == 1) . copied () ; assert_eq ! (result , Some (1)) ; assert_eq ! (items (& mut cache) , [1 , 3 , 2 , 0] , "Retrieved item is moved to front.") ; }
    };
}

find!()