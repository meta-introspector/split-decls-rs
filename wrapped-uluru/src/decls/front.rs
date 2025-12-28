macro_rules! deps {
    () => {
        TestCache!();
    };
}

macro_rules! front {
    () => {
        deps!();
        # [test] fn front () { let mut cache = TestCache :: default () ; assert_eq ! (cache . front () , None , "Nothing is in the front.") ; cache . insert (0) ; cache . insert (1) ; assert_eq ! (cache . front () , Some (& 1) , "The last inserted item should be in the front.") ; cache . touch (| x | * x == 0) ; assert_eq ! (cache . front () , Some (& 0) , "Touched item should be in the front.") ; }
    };
}

front!();