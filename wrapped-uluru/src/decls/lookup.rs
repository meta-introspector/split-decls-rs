macro_rules! deps {
    () => {
        TestCache!();
    };
}

macro_rules! lookup {
    () => {
        deps!();
        # [test] fn lookup () { let mut cache = TestCache :: default () ; cache . insert (1) ; cache . insert (2) ; cache . insert (3) ; cache . insert (4) ; let result = cache . lookup (| x | if * x == 5 { Some (()) } else { None }) ; assert_eq ! (result , None , "Cache miss.") ; assert_eq ! (items (& mut cache) , [4 , 3 , 2 , 1] , "Order not changed.") ; let result = cache . lookup (| x | if * x == 3 { Some (* x * 2) } else { None }) ; assert_eq ! (result , Some (6) , "Cache hit.") ; assert_eq ! (items (& mut cache) , [3 , 4 , 2 , 1] , "Matching item moved to front.") ; }
    };
}

lookup!();