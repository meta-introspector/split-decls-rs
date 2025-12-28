macro_rules! deps {
    () => {
        TestCache!();
    };
}

macro_rules! get {
    () => {
        deps!();
        # [test] fn get () { let mut cache = TestCache :: default () ; assert_eq ! (cache . get (0) , None , "Nothing at index 0.") ; cache . insert (42) ; cache . insert (64) ; assert_eq ! (cache . get (0) , Some (& 64) , "The last inserted item should be at index 0.") ; assert_eq ! (cache . get (1) , Some (& 42) , "The first inserted item should be at index 1.") ; cache . touch (| x | * x == 42) ; assert_eq ! (cache . get (0) , Some (& 42) , "The last touched item should be at index 0.") ; assert_eq ! (cache . get (1) , Some (& 64) , "The previously front item should be at index 1.") ; }
    };
}

get!()