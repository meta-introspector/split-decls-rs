macro_rules! deps {
    () => {
        TestCache!();
    };
}

macro_rules! touch {
    () => {
        deps!();
        # [test] fn touch () { let mut cache = TestCache :: default () ; cache . insert (0) ; cache . insert (1) ; cache . insert (2) ; cache . insert (3) ; cache . touch (| x | * x == 5) ; assert_eq ! (items (& mut cache) , [3 , 2 , 1 , 0] , "Nothing is touched.") ; cache . touch (| x | * x == 1) ; assert_eq ! (items (& mut cache) , [1 , 3 , 2 , 0] , "Touched item is moved to front.") ; }
    };
}

touch!()