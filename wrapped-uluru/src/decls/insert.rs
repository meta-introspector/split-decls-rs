macro_rules! deps {
    () => {
        TestCache!();
    };
}

macro_rules! insert {
    () => {
        deps!();
        # [test] fn insert () { let mut cache = TestCache :: default () ; cache . insert (1) ; assert_eq ! (cache . len () , 1) ; cache . insert (2) ; assert_eq ! (cache . len () , 2) ; cache . insert (3) ; assert_eq ! (cache . len () , 3) ; cache . insert (4) ; assert_eq ! (cache . len () , 4) ; assert_eq ! (items (& mut cache) , [4 , 3 , 2 , 1] , "Ordered from most- to least-recent.") ; cache . insert (5) ; assert_eq ! (cache . len () , 4) ; assert_eq ! (items (& mut cache) , [5 , 4 , 3 , 2] , "Least-recently-used item evicted.") ; cache . insert (6) ; cache . insert (7) ; cache . insert (8) ; cache . insert (9) ; assert_eq ! (items (& mut cache) , [9 , 8 , 7 , 6] , "Least-recently-used item evicted.") ; }
    };
}

insert!()