macro_rules! deps {
    () => {
        TestGraph!();
        Maxes!();
        Sccs!();
    };
}

macro_rules! test_max_branch {
    () => {
        deps!();
        # [test] fn test_max_branch () { let graph = TestGraph :: new (0 , & [(0 , 1) , (0 , 2) , (1 , 3) , (2 , 4)]) ; let mut annotations = Maxes (IndexVec :: new () , | n | n) ; let sccs = Sccs :: new_with_annotation (& graph , & mut annotations) ; assert_eq ! (annotations . 0 [sccs . scc (0)] , 4) ; assert_eq ! (annotations . 0 [sccs . scc (1)] , 3) ; assert_eq ! (annotations . 0 [sccs . scc (2)] , 4) ; }
    };
}

test_max_branch!();